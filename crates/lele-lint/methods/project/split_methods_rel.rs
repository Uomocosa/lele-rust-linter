use std::path::Path;
use std::path::PathBuf;

use crate::Project;

pub fn split_methods_rel(project: &Project, rel: &Path) -> (Option<String>, PathBuf) {
    if !project.is_multi_root() {
        return (None, rel.to_path_buf());
    }
    let Some(first) = rel.components().next().and_then(|c| c.as_os_str().to_str()) else {
        return (None, rel.to_path_buf());
    };
    if !project.layout_root_names().iter().any(|name| name == first) {
        return (None, rel.to_path_buf());
    }
    let Ok(rest) = rel.strip_prefix(first) else {
        return (None, rel.to_path_buf());
    };
    (Some(first.to_string()), rest.to_path_buf())
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::path::PathBuf;

    use super::split_methods_rel;
    use crate::Project;

    #[test]
    fn test_usage() {
        let project = Project {
            layout_roots: vec![PathBuf::from("src"), PathBuf::from("ui")],
            ..Project::default()
        };
        assert_eq!(
            split_methods_rel(&project, Path::new("src/widget/spin.rs")),
            (Some("src".to_string()), PathBuf::from("widget/spin.rs"))
        );
        assert_eq!(
            split_methods_rel(&project, Path::new("widget/spin.rs")),
            (None, PathBuf::from("widget/spin.rs"))
        );
    }

    #[test]
    fn test_usage_single_root_never_splits() {
        let project = Project::default();
        assert_eq!(
            split_methods_rel(&project, Path::new("src/widget/spin.rs")),
            (None, PathBuf::from("src/widget/spin.rs"))
        );
    }
}
