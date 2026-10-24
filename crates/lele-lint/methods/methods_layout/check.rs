use std::path::Path;

use crate::common;
use crate::rules;
use crate::Origin;
use crate::ParsedSource;
use crate::Project;
use crate::RawCode;
use crate::RuleViolation;

pub fn check(
    _self: &rules::methods_layout::MethodsLayout,
    project: &Project,
) -> Vec<RuleViolation> {
    let mut violations = Vec::new();
    let declared = project.collect_declared();

    for (type_snake, declared_type) in &declared {
        for method in &declared_type.methods {
            let rel = Path::new(type_snake).join(format!("{method}.rs"));
            match project.methods_parsed_files.get(&rel) {
                Some(file) => {
                    if !rules::methods_layout::MethodsLayout::defines_pub_fn(file, method) {
                        violations.push(rules::methods_layout::MethodsLayout::file_violation(
                            project,
                            &rel,
                            format!(
                                "`methods/{type_snake}/{method}.rs` must define `pub fn {method}`"
                            ),
                        ));
                    } else if rules::methods_layout::MethodsLayout::pub_fn_block(file, method).is_some_and(|block| block.stmts.len() <= 1)
                    {
                        violations.push(rules::methods_layout::MethodsLayout::file_violation(
                            project,
                            &rel,
                            format!(
                                "`methods/{type_snake}/{method}.rs` body has ≤1 statement — inline `pub fn {method}` in `{type_snake}.rs` under `#[rustfmt::skip]`, then delete this file"
                            ),
                        ));
                    }
                    if !(ParsedSource {
                        origin: Origin::Methods,
                        relative_path: &rel,
                        file,
                    })
                    .has_test_usage()
                        && !rules::methods_layout::MethodsLayout::read_methods_file(project, &rel)
                            .is_some_and(|text| RawCode(&text).ends_with_test_usage_opt_out())
                    {
                        violations.push(rules::methods_layout::MethodsLayout::file_violation(
                            project,
                            &rel,
                            format!(
                                "`methods/{type_snake}/{method}.rs` must carry its own `#[cfg(test)]` `test_usage`"
                            ),
                        ));
                    }
                }
                None => violations.push(rules::methods_layout::MethodsLayout::file_violation(
                    project,
                    &rel,
                    format!(
                        "missing `methods/{type_snake}/{method}.rs` for `{type_snake}::{method}` — create it with `pub fn {method}` and a `test_usage` test"
                    ),
                )),
            }
        }

        let mod_rel = Path::new(type_snake).join("mod.rs");
        let expected = declared_type.index_content();
        if project.methods_parsed_files.contains_key(&mod_rel) {
            if rules::methods_layout::MethodsLayout::read_methods_file(project, &mod_rel).as_deref()
                != Some(expected.as_str())
            {
                violations.push(rules::methods_layout::MethodsLayout::file_violation(
                    project,
                    &mod_rel,
                    format!(
                        "`methods/{type_snake}/mod.rs` is out of date — run `lele_lint --sync-methods`"
                    ),
                ));
            }
        } else {
            violations.push(rules::methods_layout::MethodsLayout::file_violation(
                project,
                &mod_rel,
                format!("missing `methods/{type_snake}/mod.rs` — run `lele_lint --sync-methods`"),
            ));
        }
    }

    for rel in project.methods_parsed_files.keys() {
        if rules::methods_layout::MethodsLayout::is_index_file(rel) {
            continue;
        }
        let Some(parent) = rules::methods_layout::MethodsLayout::parent_dir(rel) else {
            continue;
        };
        let Some(stem) = rules::methods_layout::MethodsLayout::file_stem(rel) else {
            continue;
        };
        if !declared
            .get(&parent)
            .is_some_and(|declared_type| declared_type.methods.contains(&stem))
        {
            violations.push(rules::methods_layout::MethodsLayout::file_violation(
                project,
                rel,
                format!(
                    "orphan method file `methods/{parent}/{stem}.rs` — no matching `{parent}::{stem}`"
                ),
            ));
        }
    }

    if !declared.is_empty() {
        let root_mod = Path::new("mod.rs");
        let expected = common::DeclaredType::root_index_content(&declared);
        if project.methods_parsed_files.contains_key(root_mod) {
            if rules::methods_layout::MethodsLayout::read_methods_file(project, root_mod).as_deref()
                != Some(expected.as_str())
            {
                violations.push(rules::methods_layout::MethodsLayout::file_violation(
                    project,
                    root_mod,
                    "`methods/mod.rs` is out of date — run `lele_lint --sync-methods`".to_string(),
                ));
            }
        } else {
            violations.push(rules::methods_layout::MethodsLayout::file_violation(
                project,
                root_mod,
                "missing `methods/mod.rs` — run `lele_lint --sync-methods`".to_string(),
            ));
        }
    }

    violations
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::check;
    use crate::rules;
    use crate::rules::methods_layout::MethodsLayout;
    use crate::Project;

    #[test]
    fn test_usage() {
        let rule = rules::methods_layout::MethodsLayout;
        let project = Project::default();
        assert!(check(&rule, &project).is_empty());
    }

    #[test]
    fn test_usage_missing_body_names_remediation() {
        let mut project = Project::default();
        project.parsed_files.insert(
            PathBuf::from("widget.rs"),
            syn::parse_str(
                "pub struct Widget;\n#[atomic_delegates]\nimpl Widget { pub fn spin(&self) {} }\n",
            )
            .unwrap(),
        );
        let violations = check(&MethodsLayout, &project);
        let body = violations
            .iter()
            .find(|d| d.message.contains("methods/widget/spin.rs"))
            .expect("missing-body diagnostic");
        assert!(body.message.contains("pub fn spin"));
        assert!(body.message.contains("test_usage"));
    }

    #[test]
    fn test_usage_short_body_demands_inline() {
        let mut project = Project::default();
        project.parsed_files.insert(
            PathBuf::from("widget.rs"),
            syn::parse_str(
                "pub struct Widget;\n#[atomic_delegates]\nimpl Widget { pub fn spin(&self) {} }\n",
            )
            .unwrap(),
        );
        project.methods_parsed_files.insert(
            PathBuf::from("widget/spin.rs"),
            syn::parse_str("pub fn spin(w: &mut Widget) { w.spin_once() }\n").unwrap(),
        );
        let violations = check(&MethodsLayout, &project);
        assert!(
            violations
                .iter()
                .any(|d| d.message.contains("body has ≤1 statement")),
            "expected inline diagnostic, got {violations:?}"
        );
    }

    #[test]
    fn test_usage_multi_statement_body_is_clean() {
        let mut project = Project::default();
        project.parsed_files.insert(
            PathBuf::from("widget.rs"),
            syn::parse_str(
                "pub struct Widget;\n#[atomic_delegates]\nimpl Widget { pub fn spin(&self) {} }\n",
            )
            .unwrap(),
        );
        project.methods_parsed_files.insert(
            PathBuf::from("widget/spin.rs"),
            syn::parse_str(
                "pub fn spin(w: &mut Widget) {\n    let n = w.count;\n    w.count = n + 1;\n}\n",
            )
            .unwrap(),
        );
        let violations = check(&MethodsLayout, &project);
        assert!(!violations
            .iter()
            .any(|d| d.message.contains("body has ≤1 statement")));
    }

    #[test]
    fn test_usage_trait_shell_demands_mirror_file() {
        let mut project = Project::default();
        project.parsed_files.insert(
            PathBuf::from("foo.rs"),
            syn::parse_str(
                "pub struct Foo;\n#[atomic_delegates]\nimpl Rule for Foo { fn check(&self) {} }\n",
            )
            .unwrap(),
        );
        let violations = check(&MethodsLayout, &project);
        let body = violations
            .iter()
            .find(|d| d.message.contains("methods/foo/check.rs"))
            .expect("missing-mirror diagnostic for trait shell method");
        assert!(body.message.contains("pub fn check"));
        assert!(body.message.contains("test_usage"));
    }
}
