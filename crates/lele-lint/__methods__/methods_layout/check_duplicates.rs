use crate::rules;
use crate::Project;
use crate::RuleViolation;

pub fn check_duplicates(project: &Project) -> Vec<RuleViolation> {
    let mut violations = Vec::new();
    for (type_snake, roots) in project.collect_type_roots() {
        if roots.len() < 2 {
            continue;
        }
        let names = roots.keys().cloned().collect::<Vec<_>>().join("`, `");
        let Some(rel) = roots.values().nth(1) else {
            continue;
        };
        violations.push(RuleViolation::from_file(
            project.absolute_src_path(rel),
            rules::methods_layout::MethodsLayout::CODE,
            format!(
                "type `{type_snake}` is declared in more than one source root (`{names}`) — declare each type in exactly one root"
            ),
        ));
    }
    violations
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::check_duplicates;
    use crate::Project;

    #[test]
    fn test_usage() {
        let dir = tempfile::tempdir().unwrap();
        let mut project = Project {
            root: dir.path().to_path_buf(),
            src_dir: dir.path().join("src"),
            extra_src_dirs: vec![dir.path().join("ui")],
            layout_roots: vec![PathBuf::from("src"), PathBuf::from("ui")],
            ..Project::default()
        };
        project.parsed_files.insert(
            PathBuf::from("widget.rs"),
            syn::parse_str("pub struct Widget;").unwrap(),
        );
        assert!(check_duplicates(&project).is_empty());
        project.parsed_files.insert(
            PathBuf::from("ui/widget.rs"),
            syn::parse_str("pub struct Widget;").unwrap(),
        );
        let violations = check_duplicates(&project);
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message.contains("more than one source root"));
        assert!(violations[0].message.contains("`src`, `ui`"));
    }
}
