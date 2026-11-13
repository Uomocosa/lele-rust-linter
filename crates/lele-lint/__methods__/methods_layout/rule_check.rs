use std::collections::BTreeMap;
use std::path::Path;
use std::path::PathBuf;

use crate::rules;
use crate::Project;
use crate::RuleViolation;

pub fn rule_check(
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
                            "missing `{}/{}` for `{type_snake}::{method}` — create it with `pub fn {method}` (receiver as first argument) and a `test_usage` test; a one-liner body is inlined in `{type_snake}.rs` instead, never padded",
                            project.methods_root_name(),
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

    use super::rule_check;
    use crate::rules;
    use crate::rules::methods_layout::MethodsLayout;
    use crate::Project;

    #[test]
    fn test_usage() {
        let rule = rules::methods_layout::MethodsLayout;
        let project = Project::default();
        assert!(rule_check(&rule, &project).is_empty());
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
        let violations = rule_check(&MethodsLayout, &project);
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
        let violations = rule_check(&MethodsLayout, &project);
        let body = violations
            .iter()
            .find(|d| d.message.contains("__methods__/widget/spin.rs"))
            .expect("missing-body diagnostic");
        assert!(body.message.contains("pub fn spin"));
        assert!(body.message.contains("test_usage"));
    }

    #[test]
    fn test_usage_short_body_demands_inline() {
        let dir = tempfile::tempdir().unwrap();
        let widget =
            "pub struct Widget;\n#[atomic_delegates]\nimpl Widget { pub fn spin(&self) {} }\n";
        let spin = "pub fn spin(w: &mut Widget) { w.spin_once() }\n";
        std::fs::write(dir.path().join("widget.rs"), widget).unwrap();
        std::fs::create_dir_all(dir.path().join("methods/widget")).unwrap();
        std::fs::write(dir.path().join("methods/widget/spin.rs"), spin).unwrap();
        let mut project = Project {
            root: dir.path().to_path_buf(),
            src_dir: dir.path().to_path_buf(),
            methods_dir: Some(dir.path().join("methods")),
            ..Project::default()
        };
        project
            .parsed_files
            .insert(PathBuf::from("widget.rs"), syn::parse_str(widget).unwrap());
        project.methods_parsed_files.insert(
            PathBuf::from("widget/spin.rs"),
            syn::parse_str(spin).unwrap(),
        );
        project.fill_rustfmt_answers().unwrap();
        let violations = rule_check(&MethodsLayout, &project);
        assert!(
            violations
                .iter()
                .any(|d| d.message.contains("body is a one-liner")),
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
        let violations = rule_check(&MethodsLayout, &project);
        assert!(!violations
            .iter()
            .any(|d| d.message.contains("body is a one-liner")));
    }

    #[test]
    fn test_usage_trait_shell_demands_mirror_file() {
        let mut project = Project::default();
        project.parsed_files.insert(
            PathBuf::from("foo.rs"),
            syn::parse_str(
                "pub struct Foo;\n#[atomic_delegates]\nimpl Rule for Foo { fn rule_check(&self) {} }",
            )
            .unwrap(),
        );
        let violations = rule_check(&MethodsLayout, &project);
        let body = violations
            .iter()
            .find(|d| d.message.contains("__methods__/foo/rule_check.rs"))
            .expect("missing-mirror diagnostic for trait shell method");
        assert!(body.message.contains("pub fn rule_check"));
        assert!(body.message.contains("test_usage"));
    }
}
