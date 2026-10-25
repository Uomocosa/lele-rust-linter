use lele_snake_case::ToLeleSnakeCase;
use std::collections::HashSet;

use crate::common;
use crate::rules;
use crate::ImplBlock;
use crate::ItemMarker;
use crate::Project;
use crate::RuleViolation;

pub fn check(
    _self: &rules::delegate_macro::DelegateMacro,
    project: &Project,
) -> Vec<RuleViolation> {
    let mut violations = Vec::new();
    for (rel_path, file) in &project.parsed_files {
        let Some(stem) = rel_path.file_stem().and_then(|s| s.to_str()) else {
            continue;
        };
        let Some(primary) = common::SourceFile(file).primary_type_name(stem) else {
            continue;
        };
        let main_blocks =
            rules::delegate_macro::DelegateMacro::annotated_inherent_blocks(file, &primary, false);
        let crate_blocks =
            rules::delegate_macro::DelegateMacro::annotated_inherent_blocks(file, &primary, true);
        if main_blocks > 1 || crate_blocks > 1 {
            violations.push(RuleViolation::from_file(project.src_dir.join(rel_path), rules::delegate_macro::DelegateMacro::CODE, format!(
                    "type `{primary}` has {} `#[atomic_delegates]` impl blocks — merge into one block, plus at most one block holding only `pub(crate)` shells",
                    main_blocks.saturating_add(crate_blocks)
                )));
        }
        let mut seen: HashSet<String> = HashSet::new();
        for item in &file.items {
            let syn::Item::Impl(impl_block) = item else {
                continue;
            };
            if impl_block
                .attrs
                .iter()
                .any(|attr| ItemMarker(attr).is_cfg_test())
            {
                continue;
            }
            if impl_block.trait_.is_some() {
                if ImplBlock(impl_block).self_type_last().as_deref() != Some(primary.as_str()) {
                    continue;
                }
                if impl_block
                    .attrs
                    .iter()
                    .any(|attr| ItemMarker(attr).is_atomic_delegates())
                {
                    violations.extend(
                        rules::delegate_macro::DelegateMacro::check_delegated_methods(
                            project, rel_path, &primary, impl_block, &mut seen,
                        ),
                    );
                    continue;
                }
                violations.extend(rules::delegate_macro::DelegateMacro::check_fn_shells(
                    project, rel_path, &primary, impl_block, &mut seen,
                ));
                let migratable =
                    rules::delegate_macro::DelegateMacro::migratable_idents(impl_block);
                if migratable.is_empty() {
                    continue;
                }
                let type_snake = primary.to_snake_case();
                let mirrors = migratable
                    .iter()
                    .map(|ident| format!("methods/{type_snake}/{ident}.rs"))
                    .collect::<Vec<_>>()
                    .join(", ");
                violations.push(RuleViolation::from_file(project.src_dir.join(rel_path), rules::delegate_macro::DelegateMacro::CODE, format!(
                        "trait impl for `{primary}` has unannotated shells — annotate each with `#[atomic_delegate({primary})]` or the whole block with `#[atomic_delegates]`; empty each shell body and move it to {mirrors} (one `pub fn` + `test_usage` per file)"
                    )));
                continue;
            }
            if ImplBlock(impl_block).self_type_last().as_deref() != Some(primary.as_str()) {
                continue;
            }
            if !rules::delegate_macro::DelegateMacro::has_fn(impl_block) {
                continue;
            }

            if impl_block
                .attrs
                .iter()
                .any(|attr| ItemMarker(attr).is_atomic_delegates())
            {
                violations.extend(
                    rules::delegate_macro::DelegateMacro::check_delegated_methods(
                        project, rel_path, &primary, impl_block, &mut seen,
                    ),
                );
                continue;
            }

            violations.extend(rules::delegate_macro::DelegateMacro::check_fn_shells(
                project, rel_path, &primary, impl_block, &mut seen,
            ));

            if rules::delegate_macro::DelegateMacro::all_methods_are_real_reserved(impl_block) {
                continue;
            }
            let migratable = rules::delegate_macro::DelegateMacro::migratable_idents(impl_block);
            if migratable.is_empty() {
                continue;
            }
            let type_snake = primary.to_snake_case();
            let mirrors = migratable
                .iter()
                .map(|ident| format!("methods/{type_snake}/{ident}.rs"))
                .collect::<Vec<_>>()
                .join(", ");
            let real = rules::delegate_macro::DelegateMacro::real_idents(impl_block);
            if real.is_empty() {
                violations.push(RuleViolation::from_file(project.src_dir.join(rel_path), rules::delegate_macro::DelegateMacro::CODE, format!(
                        "inherent impl for `{primary}` is not annotated with `#[atomic_delegates]` — add it, empty each body and move it to {mirrors} (one `pub fn` + `test_usage` per file)"
                    )));
            } else {
                let kept = real
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(", ");
                violations.push(RuleViolation::from_file(project.src_dir.join(rel_path), rules::delegate_macro::DelegateMacro::CODE, format!(
                        "inherent impl for `{primary}` mixes shells with hand-written methods — split it into a plain block keeping {kept} and an `#[atomic_delegates]` block with empty bodies moved to {mirrors} (one `pub fn` + `test_usage` per file)"
                    )));
            }
        }
    }
    violations
}

#[cfg(test)]
mod tests {
    use crate::rules;
    use std::path::PathBuf;

    use super::check;
    use crate::rules::delegate_macro::DelegateMacro;
    use crate::Project;

    #[test]
    fn test_usage() {
        let new_ident: syn::Ident = syn::parse_str("new").unwrap();
        let other: syn::Ident = syn::parse_str("increment").unwrap();
        assert!(rules::delegate_macro::DelegateMacro::is_reserved(
            &new_ident
        ));
        assert!(!rules::delegate_macro::DelegateMacro::is_reserved(&other));

        let real: syn::ItemImpl = syn::parse_quote! {
            impl Foo { pub const fn new() -> Self { Self } }
        };
        assert!(rules::delegate_macro::DelegateMacro::all_methods_are_real_reserved(&real));

        let delegated: syn::ItemImpl = syn::parse_quote! {
            impl Foo { pub fn new() -> Self { foo_new::new() } }
        };
        assert!(!rules::delegate_macro::DelegateMacro::all_methods_are_real_reserved(&delegated));

        let mixed: syn::ItemImpl = syn::parse_quote! {
            impl Foo { pub fn new() -> Self { Self } pub fn other(&self) {} }
        };
        assert!(!rules::delegate_macro::DelegateMacro::all_methods_are_real_reserved(&mixed));
    }

    #[test]
    fn test_usage_flags_bare_trait_shell_but_exempts_real_body() {
        let mut project = Project::default();
        project.parsed_files.insert(
            PathBuf::from("foo.rs"),
            syn::parse_str(
                "pub struct Foo;\nimpl Rule for Foo { fn check(&self, p: &Project) { rules::foo_check::check(self, p) } }\n",
            )
            .unwrap(),
        );
        let violations = check(&DelegateMacro, &project);
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message.contains("methods/foo/check.rs"));

        let mut real = Project::default();
        real.parsed_files.insert(
            PathBuf::from("bar.rs"),
            syn::parse_str(
                "pub struct Bar;\nimpl Visitor for Bar { fn visit(&self, x: u32) { let y = x + 1; visit_child(y) } }\n",
            )
            .unwrap(),
        );
        assert!(check(&DelegateMacro, &real).is_empty());
    }

    #[test]
    fn test_usage_flags_duplicate_annotated_shell() {
        let mut project = Project::default();
        project.parsed_files.insert(
            PathBuf::from("foo.rs"),
            syn::parse_str(
                "pub struct Foo;\n#[atomic_delegates]\nimpl Rule for Foo { fn check(&self) {} }\n#[atomic_delegates]\nimpl Rule for Foo { fn check(&self) {} }\n",
            )
            .unwrap(),
        );
        let violations = check(&DelegateMacro, &project);
        assert!(violations.iter().any(|d| d.message.contains("duplicate")));
    }

    #[test]
    fn test_usage_two_from_impls_are_not_duplicates() {
        let mut project = Project::default();
        project.parsed_files.insert(
            PathBuf::from("temperature.rs"),
            syn::parse_str(
                "pub struct Temperature;\n#[atomic_delegates]\nimpl From<Celsius> for Temperature { fn from(c: Celsius) -> Self {} }\n#[atomic_delegates]\nimpl From<Kelvin> for Temperature { fn from(k: Kelvin) -> Self {} }\nimpl From<Rankine> for Temperature { #[atomic_delegate(Temperature)] fn from(r: Rankine) -> Self {} }\n",
            )
            .unwrap(),
        );
        let messages: Vec<String> = check(&DelegateMacro, &project)
            .into_iter()
            .map(|v| v.message)
            .collect();
        assert!(!messages.iter().any(|m| m.contains("duplicate")));
        assert!(messages
            .iter()
            .any(|m| m.contains("methods/temperature/from_rankine.rs")));
    }

    #[test]
    fn test_usage_allows_one_extra_crate_only_block() {
        let mut project = Project::default();
        project.parsed_files.insert(
            PathBuf::from("foo.rs"),
            syn::parse_str(
                "pub struct Foo;\n#[atomic_delegates]\nimpl Foo { pub fn run(&self) {} }\n#[atomic_delegates]\nimpl Foo { pub(crate) fn step(&self) {} }\n",
            )
            .unwrap(),
        );
        let violations = check(&DelegateMacro, &project);
        assert!(!violations
            .iter()
            .any(|d| d.message.contains("merge into one block")));
        project.parsed_files.insert(
            PathBuf::from("foo.rs"),
            syn::parse_str(
                "pub struct Foo;\n#[atomic_delegates]\nimpl Foo { pub(crate) fn a(&self) {} }\n#[atomic_delegates]\nimpl Foo { pub(crate) fn b(&self) {} }\n",
            )
            .unwrap(),
        );
        let violations = check(&DelegateMacro, &project);
        assert!(violations
            .iter()
            .any(|d| d.message.contains("merge into one block")));
    }

    #[test]
    fn test_usage_flags_multiple_annotated_inherent_blocks() {
        let mut project = Project::default();
        project.parsed_files.insert(
            PathBuf::from("foo.rs"),
            syn::parse_str(
                "pub struct Foo;\n#[atomic_delegates]\nimpl Foo { pub fn run(&self) {} }\n#[atomic_delegates]\nimpl Foo { pub fn stop(&self) {} }\n",
            )
            .unwrap(),
        );
        let violations = check(&DelegateMacro, &project);
        assert!(
            violations
                .iter()
                .any(|d| d.message.contains("merge into one block")),
            "expected merge diagnostic, got {violations:?}"
        );
    }

    #[test]
    fn test_usage_single_annotated_block_is_clean() {
        let mut project = Project::default();
        project.parsed_files.insert(
            PathBuf::from("foo.rs"),
            syn::parse_str(
                "pub struct Foo;\n#[atomic_delegates]\nimpl Foo { pub fn run(&self) {} pub fn stop(&self) {} }\n",
            )
            .unwrap(),
        );
        assert!(!check(&DelegateMacro, &project)
            .iter()
            .any(|d| d.message.contains("merge into one block")));
    }

    #[test]
    fn test_usage_fn_shell_in_bare_trait_block_is_clean() {
        let mut project = Project::default();
        project.parsed_files.insert(
            PathBuf::from("foo.rs"),
            syn::parse_str(
                "pub struct Foo;\nimpl Rule for Foo { fn name(&self) -> &'static str { Self::NAME } #[atomic_delegate(Foo)] fn check(&self) {} }\n",
            )
            .unwrap(),
        );
        assert!(check(&DelegateMacro, &project).is_empty());
    }

    #[test]
    fn test_usage_fn_shell_reserved_new_is_flagged() {
        let mut project = Project::default();
        project.parsed_files.insert(
            PathBuf::from("foo.rs"),
            syn::parse_str(
                "pub struct Foo;\nimpl Foo { #[atomic_delegate(Foo)] pub fn new() -> Self {} }\n",
            )
            .unwrap(),
        );
        let violations = check(&DelegateMacro, &project);
        assert!(violations.iter().any(|d| d.message.contains("constructor")));
    }

    #[test]
    fn test_usage_cfg_test_impl_is_skipped() {
        let mut project = Project::default();
        project.parsed_files.insert(
            PathBuf::from("foo.rs"),
            syn::parse_str(
                "pub struct Foo;\n#[cfg(test)]\nimpl Foo { fn help(&self) { foo_help::help(self) } }\n",
            )
            .unwrap(),
        );
        assert!(check(&DelegateMacro, &project).is_empty());
    }

    #[test]
    fn test_usage_redundant_fn_attr_in_annotated_block() {
        let mut project = Project::default();
        project.parsed_files.insert(
            PathBuf::from("foo.rs"),
            syn::parse_str(
                "pub struct Foo;\n#[atomic_delegates]\nimpl Foo { #[atomic_delegate(Foo)] pub fn run(&self) {} }\n",
            )
            .unwrap(),
        );
        let violations = check(&DelegateMacro, &project);
        assert!(violations.iter().any(|d| d.message.contains("redundant")));
    }

    #[test]
    fn test_usage_inherent_mixed_demands_split() {
        let mut project = Project::default();
        project.parsed_files.insert(
            PathBuf::from("foo.rs"),
            syn::parse_str(
                "pub struct Foo;\nimpl Foo { fn run(&self) { foo_run::run(self) } fn compute(&self) -> i32 { let x = 1; x + 1 } }\n",
            )
            .unwrap(),
        );
        let violations = check(&DelegateMacro, &project);
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message.contains("split"));
        assert!(violations[0].message.contains("methods/foo/run.rs"));
    }

    #[test]
    fn test_usage_all_fn_shell_inherent_is_clean() {
        let mut project = Project::default();
        project.parsed_files.insert(
            PathBuf::from("foo.rs"),
            syn::parse_str(
                "pub struct Foo;\nimpl Foo { #[atomic_delegate(Foo)] pub fn run(&self) {} }\n",
            )
            .unwrap(),
        );
        assert!(check(&DelegateMacro, &project).is_empty());
    }

    #[test]
    fn test_usage_bare_trait_methods_dispatch_is_flagged() {
        let mut project = Project::default();
        project.parsed_files.insert(
            PathBuf::from("foo.rs"),
            syn::parse_str(
                "pub struct Foo;\nimpl Rule for Foo { fn check(&self) { crate::methods::foo::check(self) } }\n",
            )
            .unwrap(),
        );
        let violations = check(&DelegateMacro, &project);
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message.contains("methods/foo/check.rs"));
    }

    #[test]
    fn test_usage_bare_trait_message_suggests_fn_attr() {
        let mut project = Project::default();
        project.parsed_files.insert(
            PathBuf::from("foo.rs"),
            syn::parse_str(
                "pub struct Foo;\nimpl Rule for Foo { fn check(&self, p: &Project) { rules::foo_check::check(self, p) } }\n",
            )
            .unwrap(),
        );
        let violations = check(&DelegateMacro, &project);
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message.contains("#[atomic_delegate("));
    }

    #[test]
    fn test_usage_message_names_mirror_files() {
        let mut project = Project::default();
        project.parsed_files.insert(
            PathBuf::from("foo.rs"),
            syn::parse_str(
                "pub struct Foo;\nimpl Foo { pub fn run(&self) {} pub fn stop(&self) {} }\n",
            )
            .unwrap(),
        );
        let violations = check(&DelegateMacro, &project);
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message.contains("methods/foo/run.rs"));
        assert!(violations[0].message.contains("methods/foo/stop.rs"));
    }
}

// no test_usage necessary
