use crate::rules;
use crate::Project;
use std::path::Path;
use std::path::PathBuf;

pub fn clamp_flat_dir(ancestor: &Path, project: &Project) -> PathBuf {
    if ancestor.as_os_str().is_empty() || project.flat_dirs.is_empty() {
        return ancestor.to_path_buf();
    }
    let mut roots = project.layout_roots.clone();
    if roots.is_empty() {
        roots.push(PathBuf::from("src"));
    }
    for flat in &project.flat_dirs {
        let flat_rel = rules::flat_dir::FlatDir::crate_relative_flat_dir(project, flat);
        if flat_rel.as_os_str().is_empty() {
            continue;
        }
        let mut placed: Option<PathBuf> = None;
        let mut under_root = false;
        for root in &roots {
            let Ok(inner) = flat_rel.strip_prefix(root) else {
                continue;
            };
            under_root = true;
            if inner.as_os_str().is_empty() {
                continue;
            }
            placed = Some(if root == Path::new("src") {
                inner.to_path_buf()
            } else {
                root.join(inner)
            });
            break;
        }
        let placed = match placed {
            Some(placed) => placed,
            None if under_root => continue,
            None => flat_rel,
        };
        if ancestor.starts_with(&placed) {
            return placed.parent().map(Path::to_path_buf).unwrap_or_default();
        }
    }
    ancestor.to_path_buf()
}

#[cfg(test)]
mod tests {
    use super::clamp_flat_dir;
    use crate::rules;
    use crate::testing;
    use crate::FlatDirEntry;
    use crate::Project;
    use std::path::Path;
    use std::path::PathBuf;

    #[test]
    fn test_usage() {
        let mut project = testing::relative_project();
        project.flat_dirs = testing::flat_dir_entries();
        assert_eq!(
            clamp_flat_dir(Path::new("other/sub"), &project),
            PathBuf::from("other/sub")
        );
        assert!(clamp_flat_dir(Path::new(""), &project)
            .as_os_str()
            .is_empty());
        let bare = Project::default();
        assert_eq!(
            clamp_flat_dir(Path::new("flat/sub"), &bare),
            PathBuf::from("flat/sub")
        );
    }

    #[test]
    fn test_usage_ancestor_inside_flat_dir_clamps_to_parent() {
        let mut project = testing::relative_project();
        project.flat_dirs = testing::flat_dir_entries();
        assert!(clamp_flat_dir(Path::new("flat/sub"), &project)
            .as_os_str()
            .is_empty());
        assert!(clamp_flat_dir(Path::new("flat"), &project)
            .as_os_str()
            .is_empty());
    }

    #[test]
    fn test_usage_ancestor_inside_extra_src_flat_dir_clamps_to_parent() {
        let dir = tempfile::tempdir().unwrap();
        let root = dir.path().join("crates/rule-engine");
        std::fs::create_dir_all(root.join("__rules__")).unwrap();
        let mut project = Project {
            root: root.clone(),
            src_dir: root.join("src"),
            config_dir: dir.path().to_path_buf(),
            ..Project::default()
        };
        project.flat_dirs = vec![FlatDirEntry {
            path: PathBuf::from("crates/rule-engine/__rules__"),
            reason: "one file per item".to_string(),
        }];
        assert!(clamp_flat_dir(Path::new("__rules__"), &project)
            .as_os_str()
            .is_empty());
        assert!(clamp_flat_dir(Path::new("__rules__/sub"), &project)
            .as_os_str()
            .is_empty());
        assert_eq!(
            clamp_flat_dir(Path::new("zones"), &project),
            PathBuf::from("zones")
        );
    }

    #[test]
    fn test_usage_root_const_used_only_in_flat_dir_stays_clean() {
        use crate::traits::Rule as _;
        let base = testing::dunder_project();
        let mut project = testing::parsed_project(
            base,
            &[
                ("constants.rs", "pub const LIMIT: u32 = 1;\n"),
                ("flat/a.rs", testing::constant_user_source("LIMIT").as_str()),
                ("flat/b.rs", testing::constant_user_source("LIMIT").as_str()),
            ],
        );
        project.flat_dirs = testing::flat_dir_entries();
        let violations = rules::constants_placement::ConstantsPlacement.check(&project);
        assert!(violations.is_empty());
    }
}
