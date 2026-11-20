use crate::testing;
use crate::Project;

pub(crate) fn flat_dir_project(dir: &tempfile::TempDir, files: &[&str]) -> Project {
    std::fs::create_dir_all(dir.path().join("src/flat")).unwrap();
    let mut project = Project {
        root: dir.path().to_path_buf(),
        src_dir: dir.path().join("src"),
        ..Project::default()
    };
    project.flat_dirs = testing::flat_dir_entries();
    project.entries = files
        .iter()
        .map(|name| testing::file_entry(&dir.path().join("src"), name))
        .collect();
    project
}

#[cfg(test)]
mod tests {
    use super::flat_dir_project;

    #[test]
    fn test_usage() {
        let dir = tempfile::tempdir().unwrap();
        let project = flat_dir_project(&dir, &["flat/ok.rs"]);
        assert_eq!(project.flat_dirs.len(), 1);
        assert_eq!(project.entries.len(), 1);
    }
}
