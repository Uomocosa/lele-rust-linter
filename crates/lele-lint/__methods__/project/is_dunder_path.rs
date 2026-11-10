use std::path::Path;

use crate::Project;

pub fn is_dunder_path(project: &Project, rel_path: &Path) -> bool {
    let in_dunder_dir = project.is_in_dunder_dir(rel_path);
    let is_dunder_file = project.is_dunder_file_path(rel_path);
    in_dunder_dir || is_dunder_file
}

#[cfg(test)]
mod tests {
    use super::is_dunder_path;
    use crate::DunderPath;
    use crate::Project;
    use std::path::{Path, PathBuf};

    #[test]
    fn test_usage() {
        let project = Project {
            dunder_paths: vec![DunderPath::try_from(PathBuf::from("__basic__")).unwrap()],
            ..Project::default()
        };
        assert!(is_dunder_path(
            &project,
            Path::new("player/__basic__/enums.rs")
        ));
        assert!(!is_dunder_path(&project, Path::new("player/player.rs")));
    }
}
