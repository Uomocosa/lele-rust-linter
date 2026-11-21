use std::path::Path;

use crate::CommonArgs;
use crate::ConfigSync;
use crate::Error;
use crate::LintSession;
use crate::Linter;
use crate::Project;
use crate::RelaxedRule;

pub fn run(
    session: &LintSession,
    linter: &mut Linter,
    args: &CommonArgs,
    sync_methods: bool,
    prepare_project: impl FnOnce(&mut Project, &Path) -> Result<(), Error>,
    report_project: impl FnOnce(&Project) -> Option<i32>,
) -> i32 {
    let mut project = match session.load_project(args) {
        Ok(project) => project,
        Err(code) => return code,
    };

    if let Some(code) = session.run_sync_config(linter, args, &project) {
        return code;
    }

    let config = match session.load_config(args, &mut project, prepare_project) {
        Ok(config) => config,
        Err(code) => return code,
    };

    let snapshot = LintSession::collect_snapshot(linter);

    if let Err(error) = project.apply_layout(&config) {
        eprintln!("{}: {}", session.binary, Error::with_causes(&error));
        return 1;
    }

    let relaxed = match linter.apply_rule_settings(&config) {
        Ok(relaxed) => relaxed,
        Err(error) => {
            eprintln!("{}: {}", session.binary, Error::with_causes(&error));
            return 1;
        }
    };

    if args.config_report {
        let filtered = match LintSession::filter_report_snapshot(&snapshot, args.only.as_deref()) {
            Ok(filtered) => filtered,
            Err(unknown) => {
                eprintln!(
                    "{}: no rule with code or name `{}`",
                    session.binary,
                    unknown.join("`, `")
                );
                return 1;
            }
        };
        print!("{}", ConfigSync::render_config_report(&filtered, &relaxed));
        return 0;
    }

    if let Some(codes) = &args.only {
        let unknown = linter.retain_codes(codes);
        if !unknown.is_empty() {
            eprintln!(
                "{}: no rule with code or name `{}`",
                session.binary,
                unknown.join("`, `")
            );
            return 1;
        }
    }
    let relaxed: Vec<RelaxedRule> = relaxed
        .into_iter()
        .filter(|entry| entry.applies_to(&project))
        .collect();
    eprint!("{}", Linter::render_relaxed_banner(&relaxed));

    if let Some(code) = report_project(&project) {
        return code;
    }

    let failures = session.parse_failure_messages(&project);
    if !failures.is_empty() {
        for failure in &failures {
            eprintln!("{failure}");
        }
        return 1;
    }

    if sync_methods {
        if let Err(error) = project.sync_methods() {
            eprintln!("{}: {}", session.binary, Error::with_causes(&error));
            return 1;
        }
        return 0;
    }

    linter.run(&project)
}

#[cfg(test)]
mod tests {
    use super::run;
    use crate::testing;
    use crate::Error;
    use crate::LintSession;
    use crate::Linter;

    #[test]
    fn test_usage() {
        let base = tempfile::tempdir().unwrap();
        std::fs::write(
            base.path().join("Cargo.toml"),
            "[package]\nname='x'\nversion='0.0.0'\n",
        )
        .unwrap();
        testing::write_source_file(base.path(), "src/lib.rs");
        std::fs::write(base.path().join("lele.toml"), "[lele.project]\n").unwrap();
        let args = testing::lint_session_args(
            Some(base.path().to_path_buf()),
            Some(base.path().join("lele.toml")),
            Some(vec![String::from("no_comments")]),
        );
        let session = LintSession {
            binary: "lele_lint",
        };
        let mut linter = Linter::default();
        let called = std::cell::Cell::new(false);
        let code = run(
            &session,
            &mut linter,
            &args,
            false,
            |_, _| {
                called.set(true);
                Ok(())
            },
            |_| None,
        );
        assert!(called.get());
        assert_eq!(code, 0);
    }

    #[test]
    fn run_returns_one_when_prepare_fails() {
        let base = tempfile::tempdir().unwrap();
        std::fs::write(
            base.path().join("Cargo.toml"),
            "[package]\nname='x'\nversion='0.0.0'\n",
        )
        .unwrap();
        testing::write_source_file(base.path(), "src/lib.rs");
        std::fs::write(base.path().join("lele.toml"), "[lele.project]\n").unwrap();
        let args = testing::lint_session_args(
            Some(base.path().to_path_buf()),
            Some(base.path().join("lele.toml")),
            None,
        );
        let session = LintSession {
            binary: "lele_lint",
        };
        let mut linter = Linter::default();
        let code = run(
            &session,
            &mut linter,
            &args,
            false,
            |_, _| Err(Error::Prepare(Box::new(std::io::Error::other("boom")))),
            |_| None,
        );
        assert_eq!(code, 1);
    }

    #[test]
    fn run_returns_report_code_without_running_linter() {
        let base = tempfile::tempdir().unwrap();
        std::fs::write(
            base.path().join("Cargo.toml"),
            "[package]\nname='x'\nversion='0.0.0'\n",
        )
        .unwrap();
        testing::write_source_file(base.path(), "src/lib.rs");
        std::fs::write(base.path().join("lele.toml"), "[lele.project]\n").unwrap();
        let args = testing::lint_session_args(
            Some(base.path().to_path_buf()),
            Some(base.path().join("lele.toml")),
            None,
        );
        let session = LintSession {
            binary: "lele_lint",
        };
        let mut linter = Linter::default();
        let code = run(
            &session,
            &mut linter,
            &args,
            false,
            |_, _| Ok(()),
            |_| Some(3),
        );
        assert_eq!(code, 3);
    }
}
