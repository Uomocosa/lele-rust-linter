use crate::scan;

pub fn registers_ui(project: &scan::BevyProject, registered: &[Vec<String>]) -> bool {
    let extra = project.require_visuals();
    registered
        .iter()
        .any(|segments| scan::BevyProject::file_spawns_ui(*project, segments, &extra))
}

#[cfg(test)]
mod tests {
    use super::registers_ui;
    use crate::scan;
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
        assert!(registers_ui(&scan::BevyProject(&project), &ui));
        assert!(!registers_ui(&scan::BevyProject(&project), &unrelated));
    }
}
