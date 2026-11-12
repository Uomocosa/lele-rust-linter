use crate::rules;
use crate::Project;
use crate::RuleViolation;

pub fn entry_violations(project: &Project) -> Vec<RuleViolation> {
    let code = rules::no_collection_newtype::NoCollectionNewtype::CODE;
    let mut violations = Vec::new();
    for entry in &project.collection_wrappers {
        let path = project.root.join(&entry.file);
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
        if path.is_file()
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
        let violations =
            rules::no_collection_newtype::NoCollectionNewtype::entry_violations(&project);
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message.contains("has no reason"));
    }
}
