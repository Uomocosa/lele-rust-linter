use std::collections::HashSet;

use syn::visit::Visit;

use crate::checkers;
use crate::ItemMarker;
use crate::Project;
use crate::RuleViolation;

pub fn check(
    _self: &checkers::no_stringified_errors::NoStringifiedErrors,
    project: &Project,
) -> Vec<RuleViolation> {
    let error_enums = own_error_enums(project);
    if error_enums.is_empty() {
        return Vec::new();
    }
    let mut violations = Vec::new();
    for source in project.content_sources() {
        let mut hits = Vec::new();
        {
            let mut visitor = StringifiedErrorVisitor {
                error_enums: &error_enums,
                impl_self: None,
                is_from_impl_for_error_enum: false,
                hits: &mut hits,
            };
            visitor.visit_file(source.file);
        }
        for hit in hits {
            violations.push(RuleViolation::from_site(
                project.absolute_path(source.origin, source.relative_path),
                hit.line,
                0,
                checkers::no_stringified_errors::NoStringifiedErrors::CODE,
                format!(
                    "`{}` is stored as text in `{}` — keep the error itself: a `#[source] source` field (plus context fields) or `#[from]`",
                    hit.error_name, hit.variant
                ),
            ));
        }
    }
    violations
}

// needed helper: names of the crate's own `#[derive(Error)]` enums
fn own_error_enums(project: &Project) -> HashSet<String> {
    project
        .content_sources()
        .flat_map(|source| source.file.items.iter())
        .filter_map(|item| match item {
            syn::Item::Enum(item_enum)
                if item_enum
                    .attrs
                    .iter()
                    .any(|attr| ItemMarker(attr).derives_any_of(&["Error"])) =>
            {
                Some(item_enum.ident.to_string())
            }
            _ => None,
        })
        .collect()
}

// needed helper: one caught error stored as text
struct StringifiedErrorHit {
    line: usize,
    error_name: String,
    variant: String,
}

// needed helper: finds map_err closures, From impls and `…Error` parameters that stringify
struct StringifiedErrorVisitor<'a> {
    error_enums: &'a HashSet<String>,
    impl_self: Option<String>,
    is_from_impl_for_error_enum: bool,
    hits: &'a mut Vec<StringifiedErrorHit>,
}

impl StringifiedErrorVisitor<'_> {
    fn inspect(&mut self, error_name: &syn::Ident, body: impl FnOnce(&mut CaughtErrorScan<'_>)) {
        let mut scan = CaughtErrorScan {
            error_name: error_name.to_string(),
            error_enums: self.error_enums,
            impl_self: self.impl_self.as_deref(),
            built_variant: None,
            is_stringified: false,
            is_kept: false,
        };
        body(&mut scan);
        if let (Some(variant), true, false) =
            (scan.built_variant, scan.is_stringified, scan.is_kept)
        {
            self.hits.push(StringifiedErrorHit {
                line: error_name.span().start().line,
                error_name: scan.error_name,
                variant,
            });
        }
    }

    fn inspect_signature(&mut self, sig: &syn::Signature, block: &syn::Block) {
        for input in &sig.inputs {
            let syn::FnArg::Typed(typed) = input else {
                continue;
            };
            let syn::Pat::Ident(pat) = &*typed.pat else {
                continue;
            };
            if self.is_from_impl_for_error_enum || names_an_error_type(&typed.ty) {
                self.inspect(&pat.ident, |scan| scan.visit_block(block));
            }
        }
    }
}

impl<'ast> Visit<'ast> for StringifiedErrorVisitor<'_> {
    fn visit_expr_method_call(&mut self, call: &'ast syn::ExprMethodCall) {
        if call.method == "map_err" {
            if let Some(syn::Expr::Closure(closure)) = call.args.first() {
                if let [syn::Pat::Ident(pat)] = closure.inputs.iter().collect::<Vec<_>>().as_slice()
                {
                    self.inspect(&pat.ident, |scan| scan.visit_expr(&closure.body));
                }
            }
        }
        syn::visit::visit_expr_method_call(self, call);
    }

    fn visit_item_impl(&mut self, item: &'ast syn::ItemImpl) {
        let previous_self = self.impl_self.take();
        let previous_from = self.is_from_impl_for_error_enum;
        self.impl_self = last_ident(&item.self_ty);
        self.is_from_impl_for_error_enum = item.trait_.as_ref().is_some_and(|(_, path, _)| {
            path.segments
                .last()
                .is_some_and(|segment| segment.ident == "From")
        }) && self
            .impl_self
            .as_ref()
            .is_some_and(|name| self.error_enums.contains(name));
        syn::visit::visit_item_impl(self, item);
        self.impl_self = previous_self;
        self.is_from_impl_for_error_enum = previous_from;
    }

    fn visit_impl_item_fn(&mut self, item: &'ast syn::ImplItemFn) {
        self.inspect_signature(&item.sig, &item.block);
        syn::visit::visit_impl_item_fn(self, item);
    }

    fn visit_item_fn(&mut self, item: &'ast syn::ItemFn) {
        let previous_from = self.is_from_impl_for_error_enum;
        self.is_from_impl_for_error_enum = false;
        self.inspect_signature(&item.sig, &item.block);
        syn::visit::visit_item_fn(self, item);
        self.is_from_impl_for_error_enum = previous_from;
    }
}

// needed helper: how one caught error is used inside the code that converts it
struct CaughtErrorScan<'a> {
    error_name: String,
    error_enums: &'a HashSet<String>,
    impl_self: Option<&'a str>,
    built_variant: Option<String>,
    is_stringified: bool,
    is_kept: bool,
}

impl CaughtErrorScan<'_> {
    fn record_variant(&mut self, path: &syn::Path) {
        let segments: Vec<String> = path
            .segments
            .iter()
            .map(|segment| segment.ident.to_string())
            .collect();
        let [.., owner, variant] = segments.as_slice() else {
            return;
        };
        let owner = if owner == "Self" {
            self.impl_self.unwrap_or_default()
        } else {
            owner.as_str()
        };
        if self.error_enums.contains(owner) && self.built_variant.is_none() {
            self.built_variant = Some(format!("{owner}::{variant}"));
        }
    }

    fn is_error_name(&self, expr: &syn::Expr) -> bool {
        matches!(expr, syn::Expr::Path(path) if path.path.is_ident(&self.error_name))
    }
}

impl<'ast> Visit<'ast> for CaughtErrorScan<'_> {
    fn visit_expr_call(&mut self, call: &'ast syn::ExprCall) {
        if let syn::Expr::Path(path) = &*call.func {
            self.record_variant(&path.path);
        }
        syn::visit::visit_expr_call(self, call);
    }

    fn visit_expr_struct(&mut self, item: &'ast syn::ExprStruct) {
        self.record_variant(&item.path);
        syn::visit::visit_expr_struct(self, item);
    }

    fn visit_expr_method_call(&mut self, call: &'ast syn::ExprMethodCall) {
        if call.method == "to_string" && self.is_error_name(&call.receiver) {
            self.is_stringified = true;
            return;
        }
        syn::visit::visit_expr_method_call(self, call);
    }

    fn visit_expr_path(&mut self, path: &'ast syn::ExprPath) {
        if path.path.is_ident(&self.error_name) {
            self.is_kept = true;
        }
    }

    fn visit_macro(&mut self, mac: &'ast syn::Macro) {
        if mentions_name(mac.tokens.clone(), &self.error_name) {
            self.is_stringified = true;
        }
    }
}

// needed helper: a macro's tokens use the name as an argument or as `{name}` in a literal
fn mentions_name(tokens: proc_macro2::TokenStream, name: &str) -> bool {
    tokens.into_iter().any(|token| match token {
        proc_macro2::TokenTree::Ident(ident) => ident == name,
        proc_macro2::TokenTree::Literal(literal) => {
            let text = literal.to_string();
            text.contains(&format!("{{{name}}}")) || text.contains(&format!("{{{name}:"))
        }
        proc_macro2::TokenTree::Group(group) => mentions_name(group.stream(), name),
        proc_macro2::TokenTree::Punct(_) => false,
    })
}

// needed helper: a parameter type whose last path segment ends in `Error`
fn names_an_error_type(ty: &syn::Type) -> bool {
    match ty {
        syn::Type::Reference(reference) => names_an_error_type(&reference.elem),
        syn::Type::Path(path) => path
            .path
            .segments
            .last()
            .is_some_and(|segment| segment.ident.to_string().ends_with("Error")),
        _ => false,
    }
}

// needed helper: the last identifier of an impl's self type
fn last_ident(ty: &syn::Type) -> Option<String> {
    match ty {
        syn::Type::Path(path) => path
            .path
            .segments
            .last()
            .map(|segment| segment.ident.to_string()),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::check;
    use crate::checkers::no_stringified_errors::NoStringifiedErrors;
    use crate::Location;
    use crate::Project;

    const ERROR_ENUM: &str = "#[derive(Debug, thiserror::Error)]\npub enum Error {\n    #[error(\"x\")]\n    Io(String),\n}\n";

    fn lines(src: &str) -> Vec<usize> {
        let mut project = Project::default();
        project.parsed_files.insert(
            PathBuf::from("error.rs"),
            syn::parse_str(ERROR_ENUM).unwrap(),
        );
        project
            .parsed_files
            .insert(PathBuf::from("probe.rs"), syn::parse_str(src).unwrap());
        let mut found: Vec<usize> = check(&NoStringifiedErrors, &project)
            .into_iter()
            .map(|violation| match violation.location {
                Location::Site { line, .. } => line,
                Location::File => 0,
            })
            .collect();
        found.sort_unstable();
        found
    }

    #[test]
    fn test_usage() {
        assert_eq!(
            lines("fn f() { a().map_err(|e| Error::Io(e.to_string())); }\n"),
            vec![1]
        );
        assert_eq!(
            lines("fn f() { a().map_err(|e| Error::Io(e)); }\n"),
            Vec::<usize>::new()
        );
    }

    #[test]
    fn test_usage_flags_format_and_inline_args() {
        let src = "fn f() {\n    a().map_err(|e| Error::Io(format!(\"bad: {}\", e)));\n    \
                   b().map_err(|error| Error::Io(format!(\"bad: {error}\")));\n}\n";
        assert_eq!(lines(src), vec![2, 3]);
    }

    #[test]
    fn test_usage_flags_from_impl_and_error_helper() {
        let src = "impl From<std::io::Error> for Error {\n    \
                   fn from(e: std::io::Error) -> Self { Self::Io(e.to_string()) }\n}\n\
                   fn io(error: &std::io::Error) -> Error { Error::Io(format!(\"{error}\")) }\n";
        assert_eq!(lines(src), vec![2, 4]);
    }

    #[test]
    fn test_usage_ignores_foreign_targets_and_plain_strings() {
        let src =
            "fn parse(v: &str) -> Result<u8, String> { v.parse().map_err(|e| format!(\"{e}\")) }\n\
                   fn refused(name: &str) -> Error { Error::Io(format!(\"{name}\")) }\n\
                   fn g() { a().map_err(|e| Other::X(e.to_string())); }\n";
        assert_eq!(lines(src), Vec::<usize>::new());
    }
}

// no test_usage necessary
