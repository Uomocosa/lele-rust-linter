use crate::scan;

pub fn rows(project: scan::BevyProject) -> Vec<scan::Row> {
    let extra = project.require_visuals();
    let mut rows: Vec<scan::Row> = project
        .sources()
        .filter_map(|source| {
            let rel_path = source.relative_path;
            let file = source.file;
            let visuals = scan::ScannedFile(file).prod_visuals(&extra);
            let previews = scan::ScannedFile(file).collect_previews();
            let relevant_plugin = scan::ScannedFile(file).defines_plugin()
                && (project.dir_spawns_ui(source.origin, rel_path)
                    || project
                        .registers_ui(&scan::ScannedFile(file).plugin_registrations(&project)));
            if visuals.is_empty() && previews.is_empty() && !relevant_plugin {
                return None;
            }
            Some(scan::Row {
                file: scan::BevyProject::display_path(source.origin, rel_path),
                markers: scan::BevyProject::join_unique(
                    scan::ScannedFile(file).declared_components(),
                ),
                visual: scan::BevyProject::join_unique(
                    visuals.iter().map(|found| found.visual.clone()).collect(),
                ),
                anim: scan::BevyProject::join_unique(
                    scan::ScannedFile(file)
                        .time_drivers()
                        .into_iter()
                        .chain(scan::ScannedFile(file).input_drivers())
                        .collect(),
                ),
                kinds: previews.iter().map(|p| p.kind).collect(),
            })
        })
        .collect();
    rows.sort_by(|a, b| a.file.cmp(&b.file));
    rows
}

#[cfg(test)]
mod tests {
    use super::rows;
    use crate::scan;
    use lele_lint::Project;
    use std::path::PathBuf;

    #[test]
    fn test_usage() {
        let mut project = Project::default();
        project.parsed_files.insert(
            PathBuf::from("ui/root.rs"),
            syn::parse_str("pub fn setup(s: &mut S) { s.spawn((Node,)); }").unwrap(),
        );
        assert!(!rows(scan::BevyProject(&project)).is_empty());
    }
}
