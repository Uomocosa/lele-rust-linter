use crate::Project;
use std::path::Path;
use std::path::PathBuf;

pub fn strip_trailing_code_dunders(dir: &Path, project: &Project) -> PathBuf {
    let mut stripped = dir.to_path_buf();
    while let Some(trailing) = stripped.file_name() {
        let is_trailing_code_dunder = project.dunder_paths.iter().any(|dunder| {
            dunder.is_dunder_dir() && !dunder.is_container() && dunder.file_name() == Some(trailing)
        });
        if !is_trailing_code_dunder {
            break;
        }
        stripped.pop();
    }
    stripped
}

#[cfg(test)]
mod tests {
    use crate::rules;
    use crate::DunderPath;
    use crate::Project;
    use std::path::Path;
    use std::path::PathBuf;

    fn project() -> Project {
        Project {
            dunder_paths: DunderPath::defaults(),
            ..Project::default()
        }
    }

    #[test]
    fn test_usage() {
        let project = project();
        assert_eq!(
            rules::constants_placement::ConstantsPlacement::strip_trailing_code_dunders(
                &PathBuf::from("foo/__traits__"),
                &project,
            ),
            PathBuf::from("foo")
        );
        assert_eq!(
            rules::constants_placement::ConstantsPlacement::strip_trailing_code_dunders(
                &PathBuf::from("foo/__bevy_systems__"),
                &project,
            ),
            PathBuf::from("foo")
        );
    }

    #[test]
    fn test_usage_crate_root_code_dunder_strips_to_the_root() {
        let project = project();
        assert_eq!(
            rules::constants_placement::ConstantsPlacement::strip_trailing_code_dunders(
                &PathBuf::from("__traits__"),
                &project,
            ),
            PathBuf::new()
        );
    }

    #[test]
    fn test_usage_container_dunder_is_not_stripped() {
        let project = project();
        assert_eq!(
            rules::constants_placement::ConstantsPlacement::strip_trailing_code_dunders(
                &PathBuf::from("foo/__basic__"),
                &project,
            ),
            PathBuf::from("foo/__basic__")
        );
    }

    #[test]
    fn test_usage_plain_dirs_pass_through() {
        let project = project();
        assert_eq!(
            rules::constants_placement::ConstantsPlacement::strip_trailing_code_dunders(
                Path::new("foo"),
                &project,
            ),
            PathBuf::from("foo")
        );
    }

    #[test]
    fn test_usage_unconfigured_dunder_names_stay() {
        let project = Project::default();
        assert_eq!(
            rules::constants_placement::ConstantsPlacement::strip_trailing_code_dunders(
                &PathBuf::from("foo/__traits__"),
                &project,
            ),
            PathBuf::from("foo/__traits__")
        );
    }
}
