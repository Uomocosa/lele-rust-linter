use crate::rules;
use crate::ImplBlock;
use crate::ItemMarker;
use crate::Project;
use crate::RuleViolation;
use lele_snake_case::ToLeleSnakeCase;
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
    for method in rules::delegate_macro::DelegateMacro::fn_methods(impl_block) {
        if !method
            .attrs
            .iter()
            .any(|attr| ItemMarker(attr).is_atomic_delegate())
        {
            continue;
        }
        let ident = &method.sig.ident;
        if rules::delegate_macro::DelegateMacro::is_reserved(ident) {
            violations.push(RuleViolation::from_file(project.src_dir.join(rel_path), rules::delegate_macro::DelegateMacro::CODE, format!(
                    "`{ident}` is a constructor and must be defined in the struct file, not delegated"
                )));
        }
        let body_name = ImplBlock(impl_block).delegate_body_name(ident);
        if *ident != body_name {
            violations.push(RuleViolation::from_file(project.src_dir.join(rel_path), rules::delegate_macro::DelegateMacro::CODE, format!(
                    "`{ident}` in a generic trait impl dispatches to `methods/{}/{body_name}.rs`; annotate the whole block with `#[atomic_delegates]` instead of `#[atomic_delegate]`",
                    primary.to_snake_case()
                )));
        }
        if !seen.insert(format!("{primary}::{body_name}")) {
            violations.push(RuleViolation::from_file(project.src_dir.join(rel_path), rules::delegate_macro::DelegateMacro::CODE, format!("`{ident}` is declared twice for `{primary}` (duplicate `#[atomic_delegates]` shell) — merge into one block")));
        }
    }
    violations
}

// no test_usage necessary
