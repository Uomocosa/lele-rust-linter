use std::ffi::OsStr;
use std::path::Path;

use syn::visit::Visit;

use crate::checkers;
use crate::DunderPath;
use crate::Project;
use crate::RuleViolation;

pub fn check(
    _self: &checkers::dunder_module_name::DunderModuleName,
    project: &Project,
) -> Vec<RuleViolation> {
    let mut violations = Vec::new();

    for source in project.content_sources() {
        let mut hits = Vec::new();
        {
            let mut visitor = DunderNameVisitor {
                dunders: &project.dunder_paths,
                hits: &mut hits,
            };
            visitor.visit_file(source.file);
        }
        for hit in hits {
            violations.push(RuleViolation::from_site(
                project.absolute_path(source.origin, source.relative_path),
                hit.line,
                0,
                checkers::dunder_module_name::DunderModuleName::CODE,
                hit.message,
            ));
        }
    }

    violations
}

// needed helper: collected dunder name hit data
struct DunderNameHit {
    line: usize,
    message: String,
}

// needed helper: syn visitor collecting dunder module names and path segments
struct DunderNameVisitor<'a> {
    dunders: &'a [DunderPath],
    hits: &'a mut Vec<DunderNameHit>,
}

impl DunderNameVisitor<'_> {
    fn flag_segment(&mut self, ident: &syn::Ident) {
        let name = ident.to_string();
        if let Some(dunder) = dunder_named(self.dunders, &name) {
            self.hits.push(DunderNameHit {
                line: ident.span().start().line,
                message: format!("`{name}` in a path — write `{}`", dunder.module_name()),
            });
        }
    }
}

impl<'ast> Visit<'ast> for DunderNameVisitor<'_> {
    fn visit_item_mod(&mut self, item: &'ast syn::ItemMod) {
        let ident = item.ident.to_string();
        let target = path_attribute(&item.attrs);
        if let Some(dunder) = dunder_named(self.dunders, &ident) {
            let path = target.unwrap_or_else(|| default_path(dunder));
            self.hits.push(DunderNameHit {
                line: item.ident.span().start().line,
                message: format!(
                    "module `{ident}` keeps its dunders — declare `#[path = \"{path}\"] pub mod {};`",
                    dunder.module_name()
                ),
            });
        } else if let Some(target) = target {
            if let Some(dunder) = dunder_in_path(self.dunders, &target) {
                if ident != dunder.module_name() {
                    self.hits.push(DunderNameHit {
                        line: item.ident.span().start().line,
                        message: format!(
                            "`#[path = \"{target}\"]` points at a dunder — name the module `{}`, not `{ident}`",
                            dunder.module_name()
                        ),
                    });
                }
            }
        }
        syn::visit::visit_item_mod(self, item);
    }

    fn visit_path(&mut self, path: &'ast syn::Path) {
        for segment in &path.segments {
            self.flag_segment(&segment.ident);
        }
        syn::visit::visit_path(self, path);
    }

    fn visit_use_path(&mut self, path: &'ast syn::UsePath) {
        self.flag_segment(&path.ident);
        syn::visit::visit_use_path(self, path);
    }

    fn visit_use_name(&mut self, name: &'ast syn::UseName) {
        self.flag_segment(&name.ident);
    }

    fn visit_use_rename(&mut self, rename: &'ast syn::UseRename) {
        self.flag_segment(&rename.ident);
    }
}

// needed helper: configured dunder whose stem equals an identifier
fn dunder_named<'a>(dunders: &'a [DunderPath], ident: &str) -> Option<&'a DunderPath> {
    dunders
        .iter()
        .find(|dunder| dunder.file_stem() == Some(OsStr::new(ident)))
}

// needed helper: configured dunder a `#[path]` module file is (`__x__.rs` or `__x__/mod.rs`)
fn dunder_in_path<'a>(dunders: &'a [DunderPath], target: &str) -> Option<&'a DunderPath> {
    let target = Path::new(target);
    let module = if target.file_name() == Some(OsStr::new("mod.rs")) {
        target.parent()?.file_name()?
    } else {
        target.file_stem()?
    };
    dunder_named(dunders, module.to_str()?)
}

// needed helper: string value of a `#[path = "..."]` attribute
fn path_attribute(attrs: &[syn::Attribute]) -> Option<String> {
    let attr = attrs.iter().find(|attr| attr.path().is_ident("path"))?;
    let syn::Meta::NameValue(name_value) = &attr.meta else {
        return None;
    };
    let syn::Expr::Lit(syn::ExprLit {
        lit: syn::Lit::Str(value),
        ..
    }) = &name_value.value
    else {
        return None;
    };
    Some(value.value())
}

// needed helper: the `#[path]` a dunder module should declare
fn default_path(dunder: &DunderPath) -> String {
    let name = dunder
        .file_name()
        .and_then(OsStr::to_str)
        .unwrap_or_default();
    if dunder.is_dunder_dir() {
        format!("{name}/mod.rs")
    } else {
        name.to_string()
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::check;
    use crate::checkers::dunder_module_name::DunderModuleName;
    use crate::DunderPath;
    use crate::Project;

    fn messages(src: &str) -> Vec<String> {
        let mut project = Project {
            dunder_paths: DunderPath::defaults(),
            ..Project::default()
        };
        project
            .parsed_files
            .insert(PathBuf::from("stock/mod.rs"), syn::parse_str(src).unwrap());
        check(&DunderModuleName, &project)
            .into_iter()
            .map(|violation| violation.message)
            .collect()
    }

    #[test]
    fn test_usage() {
        assert_eq!(
            messages("#[path = \"__basic__/mod.rs\"]\npub mod basic;\n"),
            Vec::<String>::new()
        );
        let found = messages("#[path = \"__basic__/mod.rs\"]\npub mod __basic__;\n");
        assert_eq!(found.len(), 1);
        assert!(found[0].contains("pub mod basic;"));
    }

    #[test]
    fn test_usage_flags_dunder_mod_without_path() {
        let found = messages("pub mod __basic__;\n");
        assert_eq!(found.len(), 1);
        assert!(found[0].contains("#[path = \"__basic__/mod.rs\"]"));
    }

    #[test]
    fn test_usage_flags_path_to_dunder_with_other_name() {
        let found = messages("#[path = \"__basic__/mod.rs\"]\npub mod helpers;\n");
        assert_eq!(found.len(), 1);
        assert!(found[0].contains("name the module `basic`, not `helpers`"));
    }

    #[test]
    fn test_usage_allows_path_to_file_inside_dunder() {
        assert_eq!(
            messages("#[path = \"__basic__/enums.rs\"]\nmod enums;\n"),
            Vec::<String>::new()
        );
    }

    #[test]
    fn test_usage_flags_dunder_path_segments() {
        let src = "use crate::__basic__;\nuse crate::stock::__basic__::enums;\n\
                   fn f(level: __basic__::enums::Bump) {}\n";
        assert_eq!(messages(src).len(), 3);
    }

    #[test]
    fn test_usage_ignores_unconfigured_dunders() {
        assert_eq!(
            messages("pub mod __other__;\nuse crate::__other__;\n"),
            Vec::<String>::new()
        );
    }
}

// no test_usage necessary
