use std::collections::BTreeSet;
use std::fmt::Write;
use std::path::Path;

use lele_lint::Origin;

use crate::scan;

pub fn ui_inventory(project: &scan::BevyProject) -> String {
    let mut out = String::new();
    let _ = writeln!(
        out,
        "{:<44} {:<26} {:<18} {:<16} {:<3} {:<3} {:<3}",
        "file", "markers", "visual", "anim driver", "png", "mp4", "scene"
    );
    for row in &rows(*project) {
        let _ = writeln!(
            out,
            "{:<44} {:<26} {:<18} {:<16} {:<3} {:<3} {:<3}",
            row.file,
            row.markers,
            row.visual,
            row.anim,
            mark(row.kinds.contains(&scan::PreviewKind::Png)),
            mark(row.kinds.contains(&scan::PreviewKind::Mp4)),
            mark(row.kinds.contains(&scan::PreviewKind::Scene))
        );
    }
    out
}

struct Row {
    file: String,
    markers: String,
    visual: String,
    anim: String,
    kinds: BTreeSet<scan::PreviewKind>,
}

// needed helper: one inventory row per file that has UI or previews
fn rows(project: scan::BevyProject) -> Vec<Row> {
    let extra = project.require_visuals();
    let mut rows: Vec<Row> = project
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
            Some(Row {
                file: display_path(source.origin, rel_path),
                markers: join_unique(scan::ScannedFile(file).declared_components()),
                visual: join_unique(visuals.iter().map(|found| found.visual.clone()).collect()),
                anim: join_unique(
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

// needed helper: origin-prefixed display path for the table
fn display_path(origin: Origin, rel_path: &Path) -> String {
    match origin {
        Origin::Src => rel_path.display().to_string(),
        Origin::Methods => format!("methods/{}", rel_path.display()),
        Origin::Examples => format!("examples/{}", rel_path.display()),
    }
}

// needed helper: comma-joined sorted-unique idents
fn join_unique(mut items: Vec<String>) -> String {
    items.sort();
    items.dedup();
    items.join(",")
}

// needed helper: coverage x / - marker
fn mark(present: bool) -> &'static str {
    if present { "x" } else { "-" }
}

#[cfg(test)]
mod tests {
    use super::ui_inventory;
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
        let text = ui_inventory(&scan::BevyProject(&project));
        assert!(text.contains("ui/root.rs"));
        assert!(text.contains("Node"));
    }
}
