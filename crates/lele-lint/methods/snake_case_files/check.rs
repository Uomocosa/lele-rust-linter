use crate::rules;
use crate::EntryKind;
use crate::Project;
use crate::RuleViolation;

pub fn check(
    _self: &rules::snake_case_files::SnakeCaseFiles,
    project: &Project,
) -> Vec<RuleViolation> {
    let mut violations = Vec::new();

    for entry in &project.entries {
        let name = rules::snake_case_files::SnakeCaseFiles::extract_name(
            &entry.relative_path,
            entry.kind == EntryKind::Directory,
        );
        if let Some(name) = name {
            if rules::snake_case_files::SnakeCaseFiles::is_allowed_dunder(project, name) {
                continue;
            }
            if !rules::snake_case_files::SnakeCaseFiles::is_snake_case(name) {
                let kind = if entry.kind == EntryKind::Directory {
                    "directory"
                } else {
                    "filename"
                };
                violations.push(RuleViolation::from_file(entry.absolute_path.clone(), rules::snake_case_files::SnakeCaseFiles::CODE, format!(
                        "{kind} `{name}` is not snake_case — rename it to use lowercase letters, digits, and underscores",
                        kind = kind,
                        name = name
                    )));
            }
        }
    }

    violations
}

#[cfg(test)]
mod tests {
    use crate::rules;

    #[test]
    fn test_usage() {
        assert!(rules::snake_case_files::SnakeCaseFiles::is_snake_case(
            "snake_case"
        ));
        assert!(rules::snake_case_files::SnakeCaseFiles::is_snake_case(
            "player"
        ));
        assert!(rules::snake_case_files::SnakeCaseFiles::is_snake_case(
            "player_new"
        ));
        assert!(!rules::snake_case_files::SnakeCaseFiles::is_snake_case(
            "PascalCase"
        ));
        assert!(!rules::snake_case_files::SnakeCaseFiles::is_snake_case(
            "camelCase"
        ));
        assert!(!rules::snake_case_files::SnakeCaseFiles::is_snake_case(
            "__double_underscore"
        ));
        assert!(!rules::snake_case_files::SnakeCaseFiles::is_snake_case(
            "_leading"
        ));
        assert!(!rules::snake_case_files::SnakeCaseFiles::is_snake_case(
            "trailing_"
        ));
    }
}
