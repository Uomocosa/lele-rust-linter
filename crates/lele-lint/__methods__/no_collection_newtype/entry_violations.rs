use crate::config;
use crate::rules;
use crate::Project;
use crate::RuleViolation;

pub fn entry_violations(project: &Project) -> Vec<RuleViolation> {
    let code = rules::no_collection_newtype::NoCollectionNewtype::CODE;
    let config_file = project.config_base().join(config::CONFIG_FILENAME);
    let mut violations = Vec::new();
    for entry in &project.collection_wrappers {
        let path = project.config_path(&entry.file);
        if entry.reason.trim().is_empty() {
            violations.push(RuleViolation::from_file(
                path.clone(),
                code,
                format!(
                    "[[lele.lint.collection_wrappers]] entry for `{}` has no reason",
                    entry.type_name
                ),
            ));
        }
        if !path.is_file() {
            violations.push(RuleViolation::from_file(
                config_file.clone(),
                code,
                format!(
                    "stale [[lele.lint.collection_wrappers]] entry: file `{}` does not exist — delete the entry",
                    entry.file.display()
                ),
            ));
            continue;
        }
        if project.is_in_crate(&path)
            && !rules::no_collection_newtype::NoCollectionNewtype::defines_listed_wrapper(
                project, entry,
            )
        {
            violations.push(RuleViolation::from_file(
                path,
                code,
                format!(
                    "stale [[lele.lint.collection_wrappers]] entry: no collection wrapper `{}` in `{}` — delete the entry",
                    entry.type_name,
                    entry.file.display()
                ),
            ));
        }
    }
    violations
}

#[cfg(test)]
mod tests {
    use crate::rules;
    use crate::CollectionWrapperEntry;
    use crate::Project;
    use std::path::PathBuf;

    #[test]
    fn test_usage() {
        let project = Project {
            root: PathBuf::from("/crate"),
            collection_wrappers: vec![CollectionWrapperEntry {
                type_name: "Deck".to_string(),
                file: PathBuf::from("src/missing.rs"),
                reason: " ".to_string(),
            }],
            ..Project::default()
        };
        let messages: Vec<String> =
            rules::no_collection_newtype::NoCollectionNewtype::entry_violations(&project)
                .into_iter()
                .map(|v| v.message)
                .collect();
        assert_eq!(messages.len(), 2, "{messages:?}");
        assert!(messages.iter().any(|m| m.contains("has no reason")));
        assert!(messages.iter().any(|m| m.contains("does not exist")));
    }
}
