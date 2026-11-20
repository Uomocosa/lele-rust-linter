use crate::rules;
use crate::Project;
use crate::RuleViolation;

pub fn rule_check(_self: &rules::flat_dir::FlatDir, project: &Project) -> Vec<RuleViolation> {
    let mut violations = Vec::new();
    if project.flat_dirs.is_empty() {
        return violations;
    }
    let base = project.config_base();
    for flat in &project.flat_dirs {
        violations.extend(rules::flat_dir::FlatDir::validate_flat_dir(flat, &base));
        let flat_rel = rules::flat_dir::FlatDir::crate_relative_flat_dir(project, flat);
        for entry in &project.entries {
            let rel = rules::flat_dir::FlatDir::crate_relative_path(project, &entry.relative_path);
            if !rel.starts_with(&flat_rel) || rel == flat_rel {
                continue;
            }
            if rel.parent() == Some(flat_rel.as_path()) {
                continue;
            }
            violations.push(RuleViolation::from_file(
                entry.absolute_path.clone(),
                rules::flat_dir::FlatDir::CODE,
                format!(
                    "flat dir `{}` holds only files ({}); `{}` is inside a subdirectory",
                    flat.path.display(),
                    flat.reason,
                    entry.relative_path.display(),
                ),
            ));
        }
    }
    violations
}

#[cfg(test)]
mod tests {
    use super::rule_check;
    use crate::rules;
    use crate::testing;
    use crate::FlatDirEntry;
    use crate::Project;
    use std::path::PathBuf;

    #[test]
    fn test_usage() {
        let dir = tempfile::tempdir().unwrap();
        let project = testing::flat_dir_project(&dir, &["flat/ok.rs", "flat/other.rs"]);
        assert!(rule_check(&rules::flat_dir::FlatDir, &project).is_empty());
    }

    #[test]
    fn test_usage_subdir_is_a_violation() {
        let dir = tempfile::tempdir().unwrap();
        let project = testing::flat_dir_project(&dir, &["flat/ok.rs", "flat/sub/nested.rs"]);
        let violations = rule_check(&rules::flat_dir::FlatDir, &project);
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message.contains("src/flat"));
        assert!(violations[0].message.contains("one file per item"));
        assert!(violations[0].message.contains("flat/sub/nested.rs"));
    }

    #[test]
    fn test_usage_extra_src_flat_dir_subdir_is_a_violation() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path().join("crates/rule-engine");
        std::fs::create_dir_all(root.join("__rules__/__basic__")).unwrap();
        let mut project = Project {
            root: root.clone(),
            src_dir: root.join("src"),
            config_dir: dir.path().to_path_buf(),
            extra_src_dirs: vec![root.join("__rules__")],
            ..Project::default()
        };
        project.flat_dirs = vec![FlatDirEntry {
            path: PathBuf::from("crates/rule-engine/__rules__"),
            reason: "one file per item".to_string(),
        }];
        project.entries = vec![
            testing::file_entry(&root, "__rules__/ok.rs"),
            testing::file_entry(&root, "__rules__/__basic__/x.rs"),
        ];
        let violations = rule_check(&rules::flat_dir::FlatDir, &project);
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message.contains("__rules__/__basic__/x.rs"));
    }

    #[test]
    fn test_usage_no_config_is_clean() {
        let mut project = testing::relative_project();
        project.entries = vec![testing::file_entry(
            std::path::Path::new("."),
            "flat/sub/nested.rs",
        )];
        assert!(rule_check(&rules::flat_dir::FlatDir, &project).is_empty());
        assert_eq!(project.entries.len(), 1);
    }
}
