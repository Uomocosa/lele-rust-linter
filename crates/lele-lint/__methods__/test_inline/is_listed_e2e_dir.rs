use crate::Project;

pub fn is_listed_e2e_dir(project: &Project, dir: &std::path::Path) -> bool {
    let base = project.config_base();
    project.e2e_test_dirs.iter().any(|listed| {
        let listed_abs = base.join(listed);
        listed_abs == *dir
            || (std::fs::canonicalize(&listed_abs)
                .is_ok_and(|listed| std::fs::canonicalize(dir).is_ok_and(|dir| listed == dir)))
    })
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::is_listed_e2e_dir;
    use crate::rules;
    use crate::Project;

    #[test]
    fn test_usage() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join("crates/x/tests")).unwrap();
        let project = Project {
            root: dir.path().join("crates/x"),
            config_dir: dir.path().to_path_buf(),
            e2e_test_dirs: vec![PathBuf::from("crates/x/tests")],
            ..Project::default()
        };
        let tests_dir = dir.path().join("crates/x/tests");
        assert!(is_listed_e2e_dir(&project, &tests_dir));
        assert!(rules::test_inline::TestInline::is_listed_e2e_dir(
            &project, &tests_dir
        ));
        let unlisted = Project {
            root: dir.path().join("crates/x"),
            config_dir: dir.path().to_path_buf(),
            ..Project::default()
        };
        assert!(!is_listed_e2e_dir(&unlisted, &tests_dir));
    }
}
