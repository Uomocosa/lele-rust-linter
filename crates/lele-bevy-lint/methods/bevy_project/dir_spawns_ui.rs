use std::path::Path;

use lele_lint::Origin;

use crate::scan;

pub fn dir_spawns_ui(project: &scan::BevyProject, origin: Origin, rel_path: &Path) -> bool {
    let extra = project.require_visuals();
    let dir = rel_path.parent();
    project.sources().any(|source| {
        source.origin == origin
            && source.relative_path.parent() == dir
            && !scan::ScannedFile(source.file)
                .prod_visuals(&extra)
                .is_empty()
    })
}

#[cfg(test)]
mod tests {
    use super::dir_spawns_ui;
    use crate::scan;
    use lele_lint::Origin;
    use lele_lint::Project;
    use std::path::{Path, PathBuf};

    #[test]
    fn test_usage() {
        let mut project = Project::default();
        project.parsed_files.insert(
            PathBuf::from("ui/spawn.rs"),
            syn::parse_str("pub fn spawn_ui(s: &mut S) { s.spawn((Node,)); }").unwrap(),
        );
        assert!(dir_spawns_ui(
            &scan::BevyProject(&project),
            Origin::Src,
            Path::new("ui/plugin.rs")
        ));
        assert!(!dir_spawns_ui(
            &scan::BevyProject(&project),
            Origin::Src,
            Path::new("p2p/plugin.rs")
        ));
        assert!(!dir_spawns_ui(
            &scan::BevyProject(&project),
            Origin::Methods,
            Path::new("ui/plugin.rs")
        ));
    }
}
