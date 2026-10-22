use lele_lint::Project;
use lele_lint::RuleViolation;

use crate::checkers;
use crate::scan;

pub fn check(_self: &checkers::bevy_ui_mp4::BevyUiMp4, project: &Project) -> Vec<RuleViolation> {
    let graph = scan::CallGraph::from(project);
    let extra = scan::BevyProject(project).require_visuals();
    let test_only = scan::BevyProject(project).test_only_files();
    let mut violations = Vec::new();

    for source in project.sources() {
        if test_only.contains(source.relative_path) {
            continue;
        }
        let rel_path = source.relative_path;
        let file = source.file;
        let found_drivers = scan::ScannedFile(file).time_drivers();
        if found_drivers.is_empty() {
            continue;
        }
        let Some(visual) = scan::ScannedFile(file).reachable_visual(&graph, &extra) else {
            continue;
        };

        let missing = scan::ScannedFile(file).require_preview(scan::PreviewKind::Mp4, || {
            scan::BevyProject(project).violation(source.origin, rel_path, visual.line,
                checkers::bevy_ui_mp4::BevyUiMp4::CODE,
                format!(
                    "file spawns UI `{}` and drives it over time ({}) but defines no `*_ui_mp4_preview` test",
                    visual.visual,
                    found_drivers.join(", ")
                ),
            )
        });
        if missing.is_some() {
            violations.extend(missing);
            continue;
        }

        let previews = scan::ScannedFile(file).collect_previews();
        let subjects = scan::ScannedFile(file).preview_subjects(&graph, &extra);
        if let Some(preview) =
            scan::Preview::missing_subject(&previews, scan::PreviewKind::Mp4, &subjects)
        {
            violations.push(scan::BevyProject(project).violation(source.origin, rel_path, preview.line,
                checkers::bevy_ui_mp4::BevyUiMp4::CODE,
                format!(
                    "`fn {}` does not record this file's UI: its `Scene::build` must register, call or add one of `{}`, not a copy of it",
                    preview.name,
                    subjects.join("`, `")
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
            PathBuf::from("ui/sync.rs"),
            syn::parse_str(
                "pub fn setup(s: &mut S) { s.spawn((Node,)); }
                 pub fn tick(time: Res<Time>) { let _ = time.delta_secs(); }",
            )
            .unwrap(),
        );
        let violations = check(&checkers::bevy_ui_mp4::BevyUiMp4, &project);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].code, checkers::bevy_ui_mp4::BevyUiMp4::CODE);
    }
}
