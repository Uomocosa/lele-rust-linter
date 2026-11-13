use crate::rules;
use crate::ImplBlock;
use crate::ItemMarker;
use crate::Project;
use crate::RuleViolation;
use lele_snake_case::traits::ToLeleSnakeCase;
use lele_snake_case::DelegateFile;
use std::collections::HashSet;
use std::path::Path;

pub fn check_fn_shells(
    project: &Project,
    rel_path: &Path,
    primary: &str,
    impl_block: &syn::ItemImpl,
    seen: &mut HashSet<String>,
) -> Vec<RuleViolation> {
    let mut violations = Vec::new();
    let trait_name = ImplBlock(impl_block).trait_ident();
    for method in rules::delegate_macro::DelegateMacro::fn_methods(impl_block) {
        let Some(attr) = method
            .attrs
            .iter()
            .find(|attr| ItemMarker(attr).is_atomic_delegate())
        else {
            continue;
        };
        let ident = &method.sig.ident;
        if rules::delegate_macro::DelegateMacro::is_reserved(ident) {
            violations.push(RuleViolation::from_file(project.absolute_src_path(rel_path), rules::delegate_macro::DelegateMacro::CODE, format!(
                    "`{ident}` is a constructor and is delegated — define it in the struct file instead"
                )));
        }
        let body_name = ImplBlock(impl_block).delegate_body_name(ident);
        let plain_name = DelegateFile::stem(trait_name.as_deref(), &ident.to_string(), "");
        if body_name != plain_name {
            violations.push(RuleViolation::from_file(project.absolute_src_path(rel_path), rules::delegate_macro::DelegateMacro::CODE, format!(
                    "`{ident}` in a generic trait impl dispatches to `{}/{}/{body_name}.rs` — annotate the whole block with `#[atomic_delegates]` instead of `#[atomic_delegate]`",
                    project.methods_root_name(),
                    primary.to_snake_case()
                )));
        }
        let stem_differs = *ident != plain_name;
        let trait_arg = ItemMarker(attr).atomic_delegate_trait_arg();
        let wrong_trait = trait_name
            .as_deref()
            .is_some_and(|name| trait_arg.as_deref().map_or(stem_differs, |arg| arg != name));
        if wrong_trait {
            violations.push(RuleViolation::from_file(project.absolute_src_path(rel_path), rules::delegate_macro::DelegateMacro::CODE, format!(
                    "`{ident}` is a trait method and must name its trait: write `#[atomic_delegate({primary}, {})]` — trait methods dispatch to `{}/{}/{body_name}.rs`",
                    trait_name.as_deref().unwrap_or_default(),
                    project.methods_root_name(),
                    primary.to_snake_case()
                )));
        }
        if !seen.insert(format!("{primary}::{body_name}")) {
            violations.push(RuleViolation::from_file(project.absolute_src_path(rel_path), rules::delegate_macro::DelegateMacro::CODE, format!("`{ident}` is declared twice for `{primary}` (duplicate `#[atomic_delegates]` shell) — merge into one block")));
        }
    }
    violations
}

// no test_usage necessary
