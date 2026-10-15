use std::collections::BTreeSet;
use std::path::PathBuf;

use lele_lint::Project;

use crate::scan;

#[must_use]
pub fn registers_ui(project: &Project, registered: &[Vec<String>]) -> bool {
    let extra = scan::require_visuals(project);
    registered
        .iter()
        .any(|segments| file_spawns_ui(project, segments, &extra))
}

// needed helper: does the file a registered path points at spawn its UI from that fn?
fn file_spawns_ui(project: &Project, segments: &[String], extra: &BTreeSet<String>) -> bool {
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
        scan::prod_visuals(file, extra)
            .iter()
            .any(|found| found.owner == *name)
    })
}

#[cfg(test)]
mod tests {
    use super::registers_ui;
    use lele_lint::Project;
    use std::path::PathBuf;

    #[test]
    fn test_usage() {
        let mut project = Project::default();
        project.parsed_files.insert(
            PathBuf::from("hud/bevy_systems/spawn_hud.rs"),
            syn::parse_str("pub fn spawn_hud(s: &mut S) { s.spawn((Node,)); }").unwrap(),
        );
        let ui = vec![vec![
            "hud".to_string(),
            "bevy_systems".to_string(),
            "spawn_hud".to_string(),
        ]];
        let unrelated = vec![vec![
            "hud".to_string(),
            "bevy_systems".to_string(),
            "tick_hud".to_string(),
        ]];
        assert!(registers_ui(&project, &ui));
        assert!(!registers_ui(&project, &unrelated));
    }
}
