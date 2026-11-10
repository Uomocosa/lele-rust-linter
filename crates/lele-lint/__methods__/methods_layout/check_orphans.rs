use std::collections::BTreeMap;
use std::collections::BTreeSet;

use crate::common;
use crate::rules;
use crate::Project;
use crate::RuleViolation;

pub fn check_orphans(
    project: &Project,
    declared: &BTreeMap<String, common::DeclaredType>,
    shell_roots: &BTreeMap<(String, String), BTreeSet<String>>,
    multi: bool,
) -> Vec<RuleViolation> {
    let mut violations = Vec::new();
    for rel in project.methods_parsed_files.keys() {
        if rules::methods_layout::MethodsLayout::is_index_file(rel) {
            continue;
        }
        let (actual_root, inner) = if multi {
            project.split_methods_rel(rel)
        } else {
            (None, rel.clone())
        };
        let Some(parent) = rules::methods_layout::MethodsLayout::parent_dir(&inner) else {
            continue;
        };
        let Some(stem) = rules::methods_layout::MethodsLayout::file_stem(&inner) else {
            continue;
        };
        if declared
            .get(&parent)
            .is_some_and(|declared_type| declared_type.methods.contains(&stem))
        {
            if !multi {
                continue;
            }
            let expected = shell_roots.get(&(parent.clone(), stem.clone()));
            let placed = match (&actual_root, expected) {
                (Some(actual), Some(names)) => names.iter().any(|name| name == actual),
                _ => false,
            };
            if placed {
                continue;
            }
            violations.push(rules::methods_layout::MethodsLayout::file_violation(
                project,
                rel,
                rules::methods_layout::MethodsLayout::wrong_root_message(
                    project,
                    rel,
                    actual_root.as_ref(),
                    &parent,
                    expected,
                ),
            ));
            continue;
        }
        violations.push(rules::methods_layout::MethodsLayout::file_violation(
            project,
            rel,
            format!(
                "orphan method file `methods/{}` — no matching `{parent}::{stem}`",
                inner.display()
            ),
        ));
    }
    violations
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::collections::BTreeSet;
    use std::path::PathBuf;

    use super::check_orphans;
    use crate::common::DeclaredType;
    use crate::Project;

    #[test]
    fn test_usage() {
        assert!(check_orphans(
            &Project::default(),
            &BTreeMap::new(),
            &BTreeMap::new(),
            false
        )
        .is_empty());
    }

    #[test]
    fn test_usage_orphan_is_reported() {
        let mut project = Project::default();
        project.methods_parsed_files.insert(
            PathBuf::from("ghost/haunt.rs"),
            syn::parse_str("pub fn haunt() {}\n").unwrap(),
        );
        let violations = check_orphans(&project, &BTreeMap::new(), &BTreeMap::new(), false);
        assert_eq!(violations.len(), 1);
        assert!(violations[0]
            .message
            .contains("orphan method file `methods/ghost/haunt.rs`"));
    }

    #[test]
    fn test_usage_wrong_root_is_reported() {
        let mut project = Project {
            layout_roots: vec![PathBuf::from("src"), PathBuf::from("ui")],
            methods_dir: Some(PathBuf::from("__methods__")),
            ..Project::default()
        };
        project.methods_parsed_files.insert(
            PathBuf::from("ui/widget/spin.rs"),
            syn::parse_str("pub fn spin() {}\n").unwrap(),
        );
        let mut declared = BTreeMap::new();
        declared.insert(
            "widget".to_string(),
            DeclaredType {
                methods: BTreeSet::from(["spin".to_string()]),
                cfgs: Vec::new(),
            },
        );
        let mut shell_roots = BTreeMap::new();
        shell_roots.insert(
            ("widget".to_string(), "spin".to_string()),
            BTreeSet::from(["src".to_string()]),
        );
        let violations = check_orphans(&project, &declared, &shell_roots, true);
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message.contains("under root `ui`"));
        assert!(violations[0].message.contains("`__methods__/src/widget/`"));
    }
}
