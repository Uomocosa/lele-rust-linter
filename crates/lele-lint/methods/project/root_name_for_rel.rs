use std::path::Path;

use crate::Project;

pub fn root_name_for_rel(project: &Project, rel_path: &Path) -> String {
    let index = project.src_index_for_rel(rel_path);
    if let Some(name) = project.layout_root_names().get(index).cloned() {
        return name;
    }
    if index == 0 {
        return project
            .src_dir
            .file_name()
            .and_then(|name| name.to_str())
            .map_or_else(|| "src".to_string(), str::to_string);
    }
    project
        .extra_src_dirs
        .get(index.saturating_sub(1))
        .and_then(|dir| dir.file_name())
        .and_then(|name| name.to_str())
        .map_or_else(|| "src".to_string(), str::to_string)
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::path::PathBuf;

    use super::root_name_for_rel;
    use crate::Project;

    #[test]
    fn test_usage() {
        let dir = tempfile::tempdir().unwrap();
        let project = Project {
            root: dir.path().to_path_buf(),
            src_dir: dir.path().join("src"),
            extra_src_dirs: vec![dir.path().join("ui")],
            layout_roots: vec![PathBuf::from("src"), PathBuf::from("ui")],
            ..Project::default()
        };
        assert_eq!(root_name_for_rel(&project, Path::new("a.rs")), "src");
        assert_eq!(
            root_name_for_rel(&project, Path::new("ui/discovery.rs")),
            "ui"
        );
    }

    #[test]
    fn test_usage_falls_back_without_layout_roots() {
        let dir = tempfile::tempdir().unwrap();
        let project = Project {
            root: dir.path().to_path_buf(),
            src_dir: dir.path().join("src"),
            extra_src_dirs: vec![dir.path().join("ui")],
            ..Project::default()
        };
        assert_eq!(root_name_for_rel(&project, Path::new("a.rs")), "src");
        assert_eq!(
            root_name_for_rel(&project, Path::new("ui/discovery.rs")),
            "ui"
        );
    }
}
