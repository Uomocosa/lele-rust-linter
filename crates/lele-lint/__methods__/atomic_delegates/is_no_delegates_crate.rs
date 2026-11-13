use crate::Project;

pub fn is_no_delegates_crate(project: &Project) -> bool {
    let base = project.config_base();
    let Ok(root) = std::fs::canonicalize(&project.root) else {
        return false;
    };
    project.no_delegates.iter().any(|entry| {
        std::fs::canonicalize(base.join(&entry.crate_path)).is_ok_and(|listed| listed == root)
    })
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
        std::fs::create_dir_all(dir.path().join("crates/other")).unwrap();
        let entry = NoDelegatesEntry {
            crate_path: PathBuf::from("crates/listed"),
            reason: "cannot depend on the macro crate".to_string(),
        };
        let listed = Project {
            root: dir.path().join("crates/listed"),
            config_dir: dir.path().to_path_buf(),
            no_delegates: vec![entry.clone()],
            ..Project::default()
        };
        assert!(rules::atomic_delegates::AtomicDelegates::is_no_delegates_crate(&listed));
        let unlisted = Project {
            root: dir.path().join("crates/other"),
            config_dir: dir.path().to_path_buf(),
            no_delegates: vec![entry],
            ..Project::default()
        };
        assert!(!rules::atomic_delegates::AtomicDelegates::is_no_delegates_crate(&unlisted));
    }
}
