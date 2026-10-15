use lele_lint::Project;
use lele_lint::RuleViolation;

use crate::checkers;
use crate::scan;

pub fn check(_self: &checkers::bevy_ui_mp4::BevyUiMp4, project: &Project) -> Vec<RuleViolation> {
    let graph = scan::CallGraph::collect(project);
    let extra = scan::require_visuals(project);
    let mut violations = Vec::new();

    for source in project.sources() {
        let rel_path = source.relative_path;
        let file = source.file;
        let found_drivers = scan::time_drivers(file);
        if found_drivers.is_empty() {
            continue;
        }
        let Some(visual) = scan::reachable_visual(file, &graph, &extra) else {
            continue;
        };

        let missing = scan::require_preview(file, scan::PreviewKind::Mp4, || {
            scan::site_violation(
                project,
                source.origin,
                rel_path,
                visual.line,
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

        let previews = scan::collect_previews(file);
        let subjects = scan::preview_subjects(file, &graph, &extra);
        if let Some(preview) =
            scan::preview_missing_subject(&previews, scan::PreviewKind::Mp4, &subjects)
        {
            violations.push(scan::site_violation(
                project,
                source.origin,
                rel_path,
                preview.line,
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
        assert_eq!(violations[0].code, "LELEBEVY037");
    }
}
