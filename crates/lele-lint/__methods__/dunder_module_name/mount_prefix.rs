use crate::common;
use crate::Project;
use std::path::Path;

pub fn mount_prefix(rel_path: &Path, project: &Project) -> Vec<String> {
    let (_, rest) = project.split_methods_rel(rel_path);
    let source = common::SourcePath(&rest);
    let mut segments = source.module_path();
    if !source.is_index_file() {
        segments.pop();
    }
    segments
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::path::PathBuf;

    use super::mount_prefix;
    use crate::Project;

    fn names(segments: &[String]) -> Vec<String> {
        segments.to_vec()
    }

    #[test]
    fn test_usage() {
        let project = Project::default();
        assert_eq!(
            mount_prefix(Path::new("stock/mod.rs"), &project),
            names(&["stock".to_string()])
        );
        assert_eq!(
            mount_prefix(Path::new("lib.rs"), &project),
            Vec::<String>::new()
        );
        assert_eq!(
            mount_prefix(Path::new("stock/guest.rs"), &project),
            names(&["stock".to_string()])
        );
    }

    #[test]
    fn test_usage_strips_layout_roots() {
        let project = Project {
            layout_roots: vec![PathBuf::from("src"), PathBuf::from("ui")],
            ..Project::default()
        };
        assert_eq!(
            mount_prefix(Path::new("src/stock/mod.rs"), &project),
            names(&["stock".to_string()])
        );
    }
}
