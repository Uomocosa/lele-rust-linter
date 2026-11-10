use std::collections::BTreeMap;
use std::path::Path;
use std::path::PathBuf;

use crate::rules;
use crate::Project;
use crate::RuleViolation;

pub fn check(
    _self: &rules::methods_layout::MethodsLayout,
    project: &Project,
) -> Vec<RuleViolation> {
    let mut violations = Vec::new();
    let declared = project.collect_declared();
    let multi = project.is_multi_root();
    let shell_roots = if multi {
        project.collect_shell_roots()
    } else {
        BTreeMap::new()
    };

    for (type_snake, declared_type) in &declared {
        for method in &declared_type.methods {
            let expected: Vec<PathBuf> = if multi {
                shell_roots
                    .get(&(type_snake.clone(), method.clone()))
                    .map(|roots| {
                        roots
                            .iter()
                            .map(|root| {
                                Path::new(root)
                                    .join(type_snake)
                                    .join(format!("{method}.rs"))
                            })
                            .collect()
                    })
                    .unwrap_or_default()
            } else {
                vec![Path::new(type_snake).join(format!("{method}.rs"))]
            };
            for rel in &expected {
                match project.methods_parsed_files.get(rel) {
                    Some(file) => violations.extend(
                        rules::methods_layout::MethodsLayout::check_body(
                            project,
                            type_snake,
                            method,
                            rel,
                            file,
                        ),
                    ),
                    None => violations.push(rules::methods_layout::MethodsLayout::file_violation(
                        project,
                        rel,
                        format!(
                            "missing `methods/{}` for `{type_snake}::{method}` — create it with `pub fn {method}` (receiver as first argument), at least 2 statements and a `test_usage` test; a one-statement body stays inline in `{type_snake}.rs` instead",
                            rel.display()
                        ),
                    )),
                }
            }
        }

        if multi {
            violations.extend(rules::methods_layout::MethodsLayout::check_type_indexes(
                project,
                &shell_roots,
                type_snake,
            ));
        } else {
            violations.extend(rules::methods_layout::MethodsLayout::check_type_index(
                project,
                type_snake,
                declared_type,
            ));
        }
    }

    violations.extend(rules::methods_layout::MethodsLayout::check_orphans(
        project,
        &declared,
        &shell_roots,
        multi,
    ));

    if !declared.is_empty() {
        if multi {
            violations.extend(rules::methods_layout::MethodsLayout::check_root_indexes(
                project, &declared,
            ));
        } else {
            violations.extend(rules::methods_layout::MethodsLayout::check_root_index(
                project, &declared,
            ));
        }
    }

    if multi {
        violations.extend(rules::methods_layout::MethodsLayout::check_duplicates(
            project,
        ));
        violations.extend(rules::methods_layout::MethodsLayout::check_root_collisions(
            project,
        ));
    }

    violations.extend(rules::methods_layout::MethodsLayout::check_stale_methods_folder(project));

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
    fn test_usage_stale_methods_folder_reported() {
        let dir = tempfile::tempdir().unwrap();
        let methods_dir = dir.path().join("methods").join("widget");
        std::fs::create_dir_all(&methods_dir).unwrap();
        std::fs::write(methods_dir.join("spin.rs"), "pub fn spin() {}\n").unwrap();
        let mut project = Project {
            root: dir.path().to_path_buf(),
            ..Project::default()
        };
        project.parsed_files.insert(
            PathBuf::from("widget.rs"),
            syn::parse_str(
                "pub struct Widget;\n#[atomic_delegates]\nimpl Widget { pub fn spin(&self) {} }\n",
            )
            .unwrap(),
        );
        let violations = check(&MethodsLayout, &project);
        assert!(
            violations.iter().any(|d| d
                .message
                .contains("rename `methods/` to `__methods__/` and update the `#[path]` mount")),
            "expected stale-folder diagnostic, got {violations:?}"
        );
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
                "pub struct Foo;\n#[atomic_delegates]\nimpl Rule for Foo { fn check(&self) {} }",
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
