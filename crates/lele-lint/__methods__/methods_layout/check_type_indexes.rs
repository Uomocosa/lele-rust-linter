use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::path::Path;

use crate::common;
use crate::rules;
use crate::Project;
use crate::RuleViolation;

pub fn check_type_indexes(
    project: &Project,
    shell_roots: &BTreeMap<(String, String), BTreeSet<String>>,
    type_snake: &str,
) -> Vec<RuleViolation> {
    let mut violations = Vec::new();
    let mut roots: BTreeSet<&String> = BTreeSet::new();
    for ((found_type, _), names) in shell_roots {
        if found_type == type_snake {
            roots.extend(names);
        }
    }
    for root in roots {
        let scoped = common::DeclaredType {
            methods: project.methods_in_root(type_snake, root),
            cfgs: Vec::new(),
        };
        let mod_rel = Path::new(root).join(type_snake).join("mod.rs");
        let expected = scoped.index_content();
        if project.methods_parsed_files.contains_key(&mod_rel) {
            if rules::methods_layout::MethodsLayout::read_methods_file(project, &mod_rel).as_deref()
                != Some(expected.as_str())
            {
                violations.push(rules::methods_layout::MethodsLayout::file_violation(
                    project,
                    &mod_rel,
                    format!(
                        "`methods/{}` is out of date — run `lele-lint --sync-methods`",
                        mod_rel.display()
                    ),
                ));
            }
        } else {
            violations.push(rules::methods_layout::MethodsLayout::file_violation(
                project,
                &mod_rel,
                format!(
                    "missing `methods/{}` — run `lele-lint --sync-methods`",
                    mod_rel.display()
                ),
            ));
        }
    }
    violations
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::collections::BTreeSet;
    use std::path::PathBuf;

    use super::check_type_indexes;
    use crate::Project;

    #[test]
    fn test_usage() {
        let project = Project {
            layout_roots: vec![PathBuf::from("src"), PathBuf::from("ui")],
            ..Project::default()
        };
        let violations = check_type_indexes(&project, &BTreeMap::new(), "widget");
        assert!(violations.is_empty());
        let mut shell_roots = BTreeMap::new();
        shell_roots.insert(
            ("widget".to_string(), "spin".to_string()),
            BTreeSet::from(["src".to_string()]),
        );
        let violations = check_type_indexes(&project, &shell_roots, "widget");
        assert_eq!(violations.len(), 1);
        assert!(violations[0]
            .message
            .contains("missing `methods/src/widget/mod.rs`"));
    }
}
