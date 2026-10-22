use std::path::Path;

use crate::DunderPath;
use crate::Project;

pub fn is_in_dunder_dir(project: &Project, rel_path: &Path) -> bool {
    let dirs: Vec<&DunderPath> = project
        .dunder_paths
        .iter()
        .filter(|dunder| dunder.is_dunder_dir())
        .collect();
    rel_path.components().any(|component| {
        dirs.iter()
            .any(|dunder| dunder.file_name() == Some(component.as_os_str()))
    })
}

#[cfg(test)]
mod tests {
    use super::is_in_dunder_dir;
    use crate::DunderPath;
    use crate::Project;
    use std::path::Path;

    #[test]
    fn test_usage() {
        let project = Project {
            dunder_paths: DunderPath::defaults(),
            ..Project::default()
        };
        assert!(is_in_dunder_dir(
            &project,
            Path::new("player/__basic__/enums.rs")
        ));
        assert!(!is_in_dunder_dir(&project, Path::new("player/player.rs")));
    }
}
