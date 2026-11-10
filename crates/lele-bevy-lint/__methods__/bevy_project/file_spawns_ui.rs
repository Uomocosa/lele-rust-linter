use std::collections::BTreeSet;
use std::path::PathBuf;

use crate::scan;

pub fn file_spawns_ui(
    project: scan::BevyProject,
    segments: &[String],
    extra: &BTreeSet<String>,
) -> bool {
    let Some((name, modules)) = segments.split_last() else {
        return false;
    };
    let mut rel_path = PathBuf::new();
    for module in modules {
        if module.as_str() != "crate" {
            rel_path.push(module);
        }
    }
    rel_path.push(format!("{name}.rs"));
    project.parsed_files.get(&rel_path).is_some_and(|file| {
        scan::ScannedFile(file)
            .prod_visuals(extra)
            .iter()
            .any(|found| found.owner == *name)
    })
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;
    use std::path::PathBuf;

    use super::file_spawns_ui;
    use crate::scan;
    use lele_lint::Project;

    #[test]
    fn test_usage() {
        let mut project = Project::default();
        project.parsed_files.insert(
            PathBuf::from("hud/spawn_hud.rs"),
            syn::parse_str("pub fn spawn_hud(s: &mut S) { s.spawn((Node,)); }").unwrap(),
        );
        let ui = ["hud".to_string(), "spawn_hud".to_string()];
        assert!(file_spawns_ui(
            scan::BevyProject(&project),
            &ui,
            &BTreeSet::new()
        ));
        let missing = ["hud".to_string(), "tick_hud".to_string()];
        assert!(!file_spawns_ui(
            scan::BevyProject(&project),
            &missing,
            &BTreeSet::new()
        ));
    }
}
