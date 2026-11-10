use std::collections::BTreeMap;
use std::path::Path;

use crate::common;
use crate::rules;
use crate::Project;
use crate::RuleViolation;

pub fn check_root_indexes(
    project: &Project,
    declared: &BTreeMap<String, common::DeclaredType>,
) -> Vec<RuleViolation> {
    let mut violations = Vec::new();
    let root_names = project.layout_root_names();
    let glue = Path::new("mod.rs");
    let expected_glue = common::DeclaredType::glue_index_content(&root_names);
    if project.methods_parsed_files.contains_key(glue) {
        if rules::methods_layout::MethodsLayout::read_methods_file(project, glue).as_deref()
            != Some(expected_glue.as_str())
        {
            violations.push(rules::methods_layout::MethodsLayout::file_violation(
                project,
                glue,
                "`methods/mod.rs` is out of date — run `lele-lint --sync-methods`".to_string(),
            ));
        }
    } else {
        violations.push(rules::methods_layout::MethodsLayout::file_violation(
            project,
            glue,
            "missing `methods/mod.rs` — run `lele-lint --sync-methods`".to_string(),
        ));
    }
    for root in &root_names {
        let in_root: BTreeMap<String, common::DeclaredType> = declared
            .iter()
            .filter(|(type_snake, _)| !project.methods_in_root(type_snake, root).is_empty())
            .map(|(type_snake, declared_type)| (type_snake.clone(), declared_type.clone()))
            .collect();
        let mod_rel = Path::new(root).join("mod.rs");
        let expected = common::DeclaredType::root_index_content(&in_root);
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
    use std::path::PathBuf;

    use super::check_root_indexes;
    use crate::Project;

    #[test]
    fn test_usage() {
        let project = Project {
            layout_roots: vec![PathBuf::from("src"), PathBuf::from("ui")],
            ..Project::default()
        };
        let violations = check_root_indexes(&project, &BTreeMap::new());
        assert_eq!(violations.len(), 3);
        assert!(violations
            .iter()
            .any(|d| d.message.contains("missing `methods/mod.rs`")));
        assert!(violations
            .iter()
            .any(|d| d.message.contains("missing `methods/src/mod.rs`")));
        assert!(violations
            .iter()
            .any(|d| d.message.contains("missing `methods/ui/mod.rs`")));
    }
}
