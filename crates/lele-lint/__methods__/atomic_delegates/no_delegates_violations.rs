use crate::config;
use crate::rules;
use crate::Project;
use crate::RuleViolation;

pub fn no_delegates_violations(project: &Project) -> Vec<RuleViolation> {
    let code = rules::atomic_delegates::AtomicDelegates::CODE;
    let config_path = project.config_base().join(config::CONFIG_FILENAME);
    let mut violations = Vec::new();
    for entry in &project.no_delegates {
        let crate_path = entry.crate_path.display();
        if entry.reason.trim().is_empty() {
            violations.push(RuleViolation::from_file(
                config_path.clone(),
                code,
                format!("[[lele.lint.no_delegates]] entry for `{crate_path}` has no reason"),
            ));
        }
        if !project.config_base().join(&entry.crate_path).is_dir() {
            violations.push(RuleViolation::from_file(
                config_path.clone(),
                code,
                format!(
                    "stale [[lele.lint.no_delegates]] entry: crate `{crate_path}` does not exist — delete the entry"
                ),
            ));
        }
    }
    violations
}

#[cfg(test)]
mod tests {
    use crate::rules;
    use crate::NoDelegatesEntry;
    use crate::Project;
    use std::path::PathBuf;

    #[test]
    fn test_usage() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join("crates/listed")).unwrap();
        let project = Project {
            config_dir: dir.path().to_path_buf(),
            no_delegates: vec![
                NoDelegatesEntry {
                    crate_path: PathBuf::from("crates/listed"),
                    reason: " ".to_string(),
                },
                NoDelegatesEntry {
                    crate_path: PathBuf::from("crates/missing"),
                    reason: "gone".to_string(),
                },
            ],
            ..Project::default()
        };
        let violations =
            rules::atomic_delegates::AtomicDelegates::no_delegates_violations(&project);
        assert_eq!(violations.len(), 2);
        assert!(violations[0].message.contains("has no reason"));
        assert!(violations[1]
            .message
            .contains("stale [[lele.lint.no_delegates]] entry"));
    }
}
