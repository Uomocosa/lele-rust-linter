use syn::spanned::Spanned;

use crate::checkers;
use crate::ImplBlock;
use crate::ItemMarker;
use crate::Project;
use crate::RuleViolation;

const CODE: &str = checkers::constructor_naming::ConstructorNaming::CODE;

pub fn check(
    _self: &checkers::constructor_naming::ConstructorNaming,
    project: &Project,
) -> Vec<RuleViolation> {
    let mut violations = Vec::new();
    for (rel_path, file) in &project.parsed_files {
        let absolute = project.src_dir.join(rel_path);
        for item in &file.items {
            let syn::Item::Impl(impl_block) = item else {
                continue;
            };
            if impl_block.trait_.is_some()
                || impl_block
                    .attrs
                    .iter()
                    .any(|attr| ItemMarker(attr).is_cfg_test())
            {
                continue;
            }
            let Some(self_name) = ImplBlock(impl_block).self_type_last() else {
                continue;
            };
            for impl_item in &impl_block.items {
                let syn::ImplItem::Fn(method) = impl_item else {
                    continue;
                };
                let sig = &method.sig;
                if sig.receiver().is_some() || sig.ident == "new" {
                    continue;
                }
                let Some(shape) = constructed_shape(&sig.output, &self_name) else {
                    continue;
                };
                if let Some(message) = advice(sig, &self_name, shape) {
                    violations.push(RuleViolation::from_site(
                        absolute.clone(),
                        sig.fn_token.span().start().line,
                        0,
                        CODE,
                        message,
                    ));
                }
            }
        }
    }
    violations
}

// needed helper: two or more arguments building `Self` need `from_`, fallible ones `try_from_`
fn advice(sig: &syn::Signature, self_name: &str, shape: &str) -> Option<String> {
    if sig.inputs.len() < 2 {
        return None;
    }
    let prefix = if shape == "Self" {
        "from_"
    } else {
        "try_from_"
    };
    let name = sig.ident.to_string();
    let is_fallible_name = name.starts_with("try_from_");
    let is_named = match prefix {
        "from_" => name.starts_with("from_"),
        _ => is_fallible_name,
    };
    if is_named {
        return None;
    }
    let suggested = name
        .strip_prefix("try_from_")
        .or_else(|| name.strip_prefix("from_"))
        .unwrap_or(&name);
    Some(format!(
        "`{self_name}::{name}` builds `{self_name}` from {} arguments — rename it `{prefix}<what it is built from>` (e.g. `{prefix}{suggested}`); `from_*` returns `Self`, `try_from_*` returns `Result<Self, _>`/`Option<Self>`",
        sig.inputs.len()
    ))
}

// needed helper: `Self`, `Result<Self, _>` or `Option<Self>` — the wrapper name, or `Self`
fn constructed_shape(output: &syn::ReturnType, self_name: &str) -> Option<&'static str> {
    fn is_self(ty: &syn::Type, self_name: &str) -> bool {
        matches!(ty, syn::Type::Path(path)
        if path.qself.is_none()
            && path.path.segments.len() == 1
            && path.path.segments.last().is_some_and(|segment| {
                segment.arguments.is_none()
                    && (segment.ident == "Self" || segment.ident == self_name)
            }))
    }

    let syn::ReturnType::Type(_, ty) = output else {
        return None;
    };
    if is_self(ty, self_name) {
        return Some("Self");
    }
    let syn::Type::Path(path) = &**ty else {
        return None;
    };
    let segment = path.path.segments.last()?;
    let wrapper = match segment.ident.to_string().as_str() {
        "Result" => "Result",
        "Option" => "Option",
        _ => return None,
    };
    let syn::PathArguments::AngleBracketed(args) = &segment.arguments else {
        return None;
    };
    match args.args.first()? {
        syn::GenericArgument::Type(inner) if is_self(inner, self_name) => Some(wrapper),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::check;
    use crate::checkers::constructor_naming::ConstructorNaming;
    use crate::Project;

    #[test]
    fn test_usage() {
        let mut project = Project::default();
        project.parsed_files.insert(
            PathBuf::from("reading.rs"),
            syn::parse_str(
                "pub struct Reading;
                impl Reading {
                    pub fn at(sensor: &str, celsius: i32) -> Self {}
                    pub fn from_parts(sensor: &str, celsius: i32) -> Self {}
                    pub fn try_from_parts(sensor: &str, celsius: i32) -> Self {}
                    pub fn from_text(text: &str, base: u8) -> Result<Reading, Error> {}
                    pub fn try_from_text(text: &str, base: u8) -> Option<Self> {}
                    pub fn new(a: u8, b: u8) -> Self {}
                    pub fn single(a: u8) -> Self {}
                    pub fn sensor(&self, a: u8) -> Self {}
                }",
            )
            .unwrap(),
        );
        let messages: Vec<String> = check(&ConstructorNaming, &project)
            .into_iter()
            .map(|v| v.message)
            .collect();
        assert_eq!(messages.len(), 3, "{messages:?}");
        assert!(messages[0].contains("e.g. `from_at`"));
        assert!(messages[1].contains("e.g. `from_parts`"));
        assert!(messages[2].contains("e.g. `try_from_text`"));
    }
}
