use std::path::Path;

use crate::Project;

pub fn src_index_for_rel(project: &Project, rel_path: &Path) -> usize {
    for (offset, extra_abs) in project.extra_src_dirs.iter().enumerate() {
        let extra_rel = extra_abs
            .strip_prefix(&project.root)
            .unwrap_or(extra_abs.as_path());
        if rel_path.starts_with(extra_rel) {
            return offset.saturating_add(1);
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::src_index_for_rel;
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
        assert_eq!(src_index_for_rel(&project, Path::new("a.rs")), 0);
        assert_eq!(src_index_for_rel(&project, Path::new("ui/discovery.rs")), 1);
    }
}
