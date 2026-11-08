use std::path::Path;
use std::path::PathBuf;

use crate::Project;

pub fn absolute_src_path(project: &Project, rel_path: &Path) -> PathBuf {
    let index = project.src_index_for_rel(rel_path);
    if index == 0 {
        project.src_dir.join(rel_path)
    } else {
        project.root.join(rel_path)
    }
}

#[cfg(test)]
mod tests {
    use super::absolute_src_path;
    use crate::Project;
    use std::path::Path;

    #[test]
    fn test_usage() {
        let dir = tempfile::tempdir().unwrap();
        let project = Project {
            root: dir.path().to_path_buf(),
            src_dir: dir.path().join("src"),
            extra_src_dirs: vec![dir.path().join("ui")],
            ..Project::default()
        };
        assert_eq!(
            absolute_src_path(&project, Path::new("a.rs")),
            dir.path().join("src").join("a.rs")
        );
        assert_eq!(
            absolute_src_path(&project, Path::new("ui/b.rs")),
            dir.path().join("ui/b.rs")
        );
    }
}
