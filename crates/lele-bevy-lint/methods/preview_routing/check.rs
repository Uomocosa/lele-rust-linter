use lele_lint::Project;
use lele_lint::RuleViolation;

use crate::checkers;
use crate::scan;

pub fn check(
    _self: &checkers::preview_routing::PreviewRouting,
    project: &Project,
) -> Vec<RuleViolation> {
    let mut violations = Vec::new();
    for source in project.sources() {
        let rel_path = source.relative_path;
        let file = source.file;
        for preview in scan::collect_previews(file) {
            if preview.is_routed_through_harness {
                continue;
            }
            violations.push(scan::site_violation(project, source.origin, rel_path,
                preview.line,
                checkers::preview_routing::PreviewRouting::CODE,
                format!(
                    "`fn {}` is a preview test but does not call `lele_bevy_preview::run(...)`; route it through the harness so an empty frame fails",
                    preview.name
                ),
            ));
        }
    }
    violations
}

#[cfg(test)]
mod tests {
    use super::check;
    use crate::checkers;
    use lele_lint::Project;
    use std::path::PathBuf;

    #[test]
    fn test_usage() {
        let mut project = Project::default();
        project.parsed_files.insert(
            PathBuf::from("ui/root.rs"),
            syn::parse_str(
                "#[cfg(test)]
                 mod tests {
                     #[test]
                     fn spawn_root_ui_png_preview() { assert!(true); }
                 }",
            )
            .unwrap(),
        );
        let violations = check(&checkers::preview_routing::PreviewRouting, &project);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].code, "E039");
    }
}
