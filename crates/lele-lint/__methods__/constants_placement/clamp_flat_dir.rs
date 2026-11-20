use crate::rules;
use crate::Project;
use std::path::Path;
use std::path::PathBuf;

pub fn clamp_flat_dir(ancestor: &Path, project: &Project) -> PathBuf {
    if ancestor.as_os_str().is_empty() || project.flat_dirs.is_empty() {
        return ancestor.to_path_buf();
    }
    let base = project.config_base();
    let prefix = rules::boundary_imports::BoundaryImports::base_prefix(project, &base);
    let mut roots = project.layout_roots.clone();
    if roots.is_empty() {
        roots.push(PathBuf::from("src"));
    }
    for flat in &project.flat_dirs {
        for root in &roots {
            let Ok(inner) = flat.path.strip_prefix(prefix.join(root)) else {
                continue;
            };
            if inner.as_os_str().is_empty() {
                continue;
            }
            let placed = if root == Path::new("src") {
                inner.to_path_buf()
            } else {
                root.join(inner)
            };
            if ancestor.starts_with(&placed) {
                return placed.parent().map(Path::to_path_buf).unwrap_or_default();
            }
        }
    }
    ancestor.to_path_buf()
}

#[cfg(test)]
mod tests {
    use super::clamp_flat_dir;
    use crate::rules;
    use crate::testing;
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
