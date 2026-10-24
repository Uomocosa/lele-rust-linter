use std::fmt::Write;

use crate::scan;

pub fn ui_inventory(project: &scan::BevyProject) -> String {
    let mut out = String::new();
    let _ = writeln!(
        out,
        "{:<44} {:<26} {:<18} {:<16} {:<3} {:<3} {:<3}",
        "file", "markers", "visual", "anim driver", "png", "mp4", "scene"
    );
    for row in &scan::BevyProject::rows(*project) {
        let _ = writeln!(
            out,
            "{:<44} {:<26} {:<18} {:<16} {:<3} {:<3} {:<3}",
            row.file,
            row.markers,
            row.visual,
            row.anim,
            scan::BevyProject::mark(row.kinds.contains(&scan::PreviewKind::Png)),
            scan::BevyProject::mark(row.kinds.contains(&scan::PreviewKind::Mp4)),
            scan::BevyProject::mark(row.kinds.contains(&scan::PreviewKind::Scene))
        );
    }
    out
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
