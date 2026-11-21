use crate::CommonArgs;
use crate::ConfigSync;
use crate::Error;
use crate::LintSession;
use crate::Linter;
use crate::Project;

pub fn run_sync_config(
    session: &LintSession,
    linter: &Linter,
    args: &CommonArgs,
    project: &Project,
) -> Option<i32> {
    if !args.sync.sync_config {
        return None;
    }
    let config_path = ConfigSync::config_path(args.config.as_deref(), project);
    let outcome = match ConfigSync::synchronize(&linter.rules, &config_path, args.sync.check) {
        Ok(outcome) => outcome,
        Err(error) => {
            eprintln!("{}: {}", session.binary, Error::with_causes(&error));
            return Some(1);
        }
    };
    if args.sync.check {
        let hyphen = session.binary.replace('_', "-");
        let report = ConfigSync::render_sync_check(&outcome, &hyphen);
        if report.is_empty() {
            return Some(0);
        }
        eprint!("{report}");
        return Some(1);
    }
    Some(0)
}

#[cfg(test)]
mod tests {
    use super::run_sync_config;
    use crate::testing;
    use crate::LintSession;
    use crate::Linter;
    use crate::Project;

    #[test]
    fn test_usage() {
        let session = LintSession {
            binary: "lele_lint",
        };
        let linter = Linter::default();
        let args = testing::lint_session_args(None, None, None);
        assert_eq!(
            run_sync_config(&session, &linter, &args, &Project::default()),
            None
        );
    }

    #[test]
    fn stale_config_check_reports_without_writing() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("lele.toml");
        std::fs::write(&path, "[lele.project]\n").unwrap();
        let session = LintSession {
            binary: "lele_lint",
        };
        let linter = Linter::default();
        let project = Project {
            config_dir: dir.path().to_path_buf(),
            ..Project::default()
        };
        let mut args = testing::lint_session_args(None, None, None);
        args.sync.sync_config = true;
        args.sync.check = true;
        assert_eq!(run_sync_config(&session, &linter, &args, &project), Some(1));
        assert_eq!(std::fs::read_to_string(&path).unwrap(), "[lele.project]\n");
    }

    #[test]
    fn synced_config_check_returns_zero() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("lele.toml");
        let session = LintSession {
            binary: "lele_lint",
        };
        let linter = Linter::default();
        let project = Project {
            root: dir.path().to_path_buf(),
            ..Project::default()
        };
        let mut sync = testing::lint_session_args(None, Some(path.clone()), None);
        sync.sync.sync_config = true;
        assert_eq!(run_sync_config(&session, &linter, &sync, &project), Some(0));
        let mut check = testing::lint_session_args(None, Some(path), None);
        check.sync.sync_config = true;
        check.sync.check = true;
        assert_eq!(
            run_sync_config(&session, &linter, &check, &project),
            Some(0)
        );
    }
}
