use crate::CommonArgs;
use crate::Error;
use crate::LintSession;
use crate::Project;

pub fn load_project(session: &LintSession, args: &CommonArgs) -> Result<Project, i32> {
    match Project::try_from_start_dir_and_scan_folders(
        args.path.as_deref(),
        args.scan_folder.as_deref(),
    ) {
        Ok(project) => Ok(project),
        Err(error) => {
            eprintln!("{}: {}", session.binary, Error::with_causes(&error));
            Err(1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::load_project;
    use crate::testing;
    use crate::LintSession;

    #[test]
    fn test_usage() {
        let base = tempfile::tempdir().unwrap();
        std::fs::write(
            base.path().join("Cargo.toml"),
            "[package]\nname='x'\nversion='0.0.0'\n",
        )
        .unwrap();
        testing::write_source_file(base.path(), "src/a.rs");
        let session = LintSession {
            binary: "lele_lint",
        };
        let args = testing::lint_session_args(Some(base.path().to_path_buf()), None, None);
        assert!(load_project(&session, &args).is_ok());
    }

    #[test]
    fn missing_cargo_root_is_rejected() {
        let base = tempfile::tempdir().unwrap();
        let session = LintSession {
            binary: "lele_lint",
        };
        let missing = base.path().join("no-such-dir");
        let args = testing::lint_session_args(Some(missing), None, None);
        assert!(load_project(&session, &args).is_err());
    }
}
