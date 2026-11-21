use crate::LintSession;
use crate::Project;

pub fn parse_failure_messages(session: &LintSession, project: &Project) -> Vec<String> {
    project
        .parse_failures
        .iter()
        .map(|failure| {
            format!(
                "{}: could not parse {}: {}",
                session.binary,
                failure.path.display(),
                failure.cause
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::parse_failure_messages;
    use crate::LintSession;
    use crate::ParseFailure;
    use crate::Project;

    #[test]
    fn test_usage() {
        let session = LintSession {
            binary: "lele_lint",
        };
        let project = Project {
            parse_failures: vec![ParseFailure {
                path: std::path::PathBuf::from("src/broken.rs"),
                cause: String::from("expected an item"),
            }],
            ..Project::default()
        };
        assert_eq!(
            parse_failure_messages(&session, &project),
            vec![String::from(
                "lele_lint: could not parse src/broken.rs: expected an item"
            )]
        );
    }

    #[test]
    fn clean_project_has_no_messages() {
        let session = LintSession {
            binary: "lele_lint",
        };
        assert_eq!(
            parse_failure_messages(&session, &Project::default()),
            Vec::<String>::new()
        );
    }
}
