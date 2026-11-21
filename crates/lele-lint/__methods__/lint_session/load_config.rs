use std::path::Path;

use crate::CommonArgs;
use crate::Config;
use crate::Error;
use crate::LintSession;
use crate::Project;

pub fn load_config(
    session: &LintSession,
    args: &CommonArgs,
    project: &mut Project,
    prepare_project: impl FnOnce(&mut Project, &Path) -> Result<(), Error>,
) -> Result<Config, i32> {
    let (config, config_dir) = match Config::resolve(args.config.as_deref(), project) {
        Ok(resolved) => resolved,
        Err(error) => {
            eprintln!(
                "{}: lele.toml: {}",
                session.binary,
                Error::with_causes(&error)
            );
            return Err(1);
        }
    };
    if let Err(error) = prepare_project(project, &config_dir) {
        eprintln!("{}: {}", session.binary, Error::with_causes(&error));
        return Err(1);
    }
    project.config_dir = config_dir;
    Ok(config)
}

#[cfg(test)]
mod tests {
    use super::load_config;
    use crate::testing;
    use crate::Error;
    use crate::LintSession;
    use crate::Project;

    #[test]
    fn test_usage() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("lele.toml");
        std::fs::write(&path, "[lele.project]\n").unwrap();
        let session = LintSession {
            binary: "lele_lint",
        };
        let mut project = Project::default();
        let args = testing::lint_session_args(None, Some(path), None);
        let config = load_config(&session, &args, &mut project, |_, _| Ok(()));
        assert!(config.is_ok());
        assert_eq!(project.config_dir, dir.path().to_path_buf());
    }

    #[test]
    fn prepare_failure_is_rejected() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("lele.toml");
        std::fs::write(&path, "[lele.project]\n").unwrap();
        let session = LintSession {
            binary: "lele_lint",
        };
        let mut project = Project::default();
        let args = testing::lint_session_args(None, Some(path), None);
        let result = load_config(&session, &args, &mut project, |_, _| {
            Err(Error::Prepare(Box::new(std::io::Error::other("boom"))))
        });
        assert_eq!(result.map(|_| ()), Err(1));
    }
}
