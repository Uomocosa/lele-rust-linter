use crate::rules;
use crate::ImplBlock;
use crate::ItemMarker;
use crate::Project;
use crate::RuleViolation;
use std::collections::HashSet;
use std::path::Path;

pub fn check_delegated_methods(
    project: &Project,
    rel_path: &Path,
    primary: &str,
    impl_block: &syn::ItemImpl,
    seen: &mut HashSet<String>,
) -> Vec<RuleViolation> {
    let mut violations = Vec::new();
    for method in rules::delegate_macro::DelegateMacro::fn_methods(impl_block) {
        let ident = &method.sig.ident;
        if rules::delegate_macro::DelegateMacro::is_reserved(ident) {
            violations.push(RuleViolation::from_file(project.src_dir.join(rel_path), rules::delegate_macro::DelegateMacro::CODE, format!(
                    "`{ident}` is a constructor and must be defined in the struct file, not delegated"
                )));
        }
        if method
            .attrs
            .iter()
            .any(|attr| ItemMarker(attr).is_atomic_delegate())
        {
            violations.push(RuleViolation::from_file(project.src_dir.join(rel_path), rules::delegate_macro::DelegateMacro::CODE, format!(
                    "`{ident}` carries a redundant `#[atomic_delegate]` — the whole block is already `#[atomic_delegates]`"
                )));
        }
        let body_name = ImplBlock(impl_block).delegate_body_name(ident);
        if !seen.insert(format!("{primary}::{body_name}")) {
            violations.push(RuleViolation::from_file(project.src_dir.join(rel_path), rules::delegate_macro::DelegateMacro::CODE, format!("`{ident}` is declared twice for `{primary}` (duplicate `#[atomic_delegates]` shell) — merge into one block")));
        }
    }
    violations
}

// no test_usage necessary
