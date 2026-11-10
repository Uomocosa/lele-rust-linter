use std::path::Path;

use crate::Project;

pub fn is_dunder_file_path(project: &Project, rel_path: &Path) -> bool {
    let file_stem = rel_path.file_stem();
    project
        .dunder_paths
        .iter()
        .any(|dunder| dunder.is_dunder_file() && dunder.file_stem() == file_stem)
}

#[cfg(test)]
mod tests {
    use super::is_dunder_file_path;
    use crate::DunderPath;
    use crate::Project;
    use std::path::{Path, PathBuf};

    #[test]
    fn test_usage() {
        let project = Project {
            dunder_paths: vec![DunderPath::try_from(PathBuf::from("__prelude__.rs")).unwrap()],
            ..Project::default()
        };
        assert!(is_dunder_file_path(
            &project,
            Path::new("player/__prelude__.rs")
        ));
        assert!(!is_dunder_file_path(
            &project,
            Path::new("player/player.rs")
        ));
    }
}
