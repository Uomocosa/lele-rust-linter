use crate::Project;

pub(crate) fn stale_methods_project(methods_root: Option<&str>) -> (tempfile::TempDir, Project) {
    let dir = tempfile::tempdir().unwrap();
    let folder = dir.path().join("methods");
    std::fs::create_dir_all(&folder).unwrap();
    std::fs::write(folder.join("mod.rs"), "pub mod player;\n").unwrap();
    let mut project = Project {
        root: dir.path().to_path_buf(),
        ..Project::default()
    };
    if let Some(methods_root) = methods_root {
        project.methods_dir = Some(dir.path().join(methods_root));
    }
    (dir, project)
}

#[cfg(test)]
mod tests {
    use super::stale_methods_project;

    #[test]
    fn test_usage() {
        let (_dir, project) = stale_methods_project(None);
        assert!(project.root.join("methods").exists());
    }
}
