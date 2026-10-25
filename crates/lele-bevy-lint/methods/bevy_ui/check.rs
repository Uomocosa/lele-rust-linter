use lele_lint::Project;
use lele_lint::RuleViolation;

use crate::rules;
use crate::scan;

pub fn check(_self: &rules::bevy_ui::BevyUi, project: &Project) -> Vec<RuleViolation> {
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
        let Some(visual) = scan::ScannedFile(file).reachable_visual(&graph, &extra) else {
            continue;
        };

        let previews = scan::ScannedFile(file).collect_previews();
        if !previews.iter().any(|p| p.kind == scan::PreviewKind::Png) {
            violations.push(scan::BevyProject(project).violation(source.origin, rel_path, visual.line,
                rules::bevy_ui::BevyUi::CODE,
                format!(
                    "file spawns UI `{}` reachable from production but defines no `*_ui_png_preview` test that renders it through `lele_bevy_preview::run` — add one (see the bevy-ui-preview skill)",
                    visual.visual
                ),
            ));
            continue;
        }

        let subjects = scan::ScannedFile(file).preview_subjects(&graph, &extra);
        if let Some(preview) =
            scan::Preview::missing_subject(&previews, scan::PreviewKind::Png, &subjects)
        {
            violations.push(scan::BevyProject(project).violation(source.origin, rel_path, preview.line,
                rules::bevy_ui::BevyUi::CODE,
                format!(
                    "`fn {}` does not render this file's UI: its `Scene::build` must register, call or add one of `{}`, not a copy of it",
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
    use crate::rules;
    use lele_lint::Project;
    use std::path::PathBuf;

    fn violations(source: &str) -> Vec<lele_lint::RuleViolation> {
        let mut project = Project::default();
        project
            .parsed_files
            .insert(PathBuf::from("ui/root.rs"), syn::parse_str(source).unwrap());
        check(&rules::bevy_ui::BevyUi, &project)
    }

    #[test]
    fn test_usage() {
        let found = violations("pub fn setup(s: &mut S) { s.spawn((Node,)); }");
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].code, rules::bevy_ui::BevyUi::CODE);
    }

    #[test]
    fn test_usage_preview_of_the_real_spawner_is_clean() {
        let found = violations(
            "pub fn setup(s: &mut S) { s.spawn((Node,)); }
             #[cfg(test)]
             mod tests {
                 use lele_bevy_preview::run;
                 fn build(app: &mut App) { app.add_systems(Startup, super::setup); }
                 #[test]
                 fn spawn_root_ui_png_preview() {
                     let scene = Scene { build, states: vec![] };
                     run(&scene, &Config::default(), \"x\").expect(\"preview\");
                 }
             }",
        );
        assert!(found.is_empty());
    }

    #[test]
    fn test_usage_preview_of_a_copy_is_flagged() {
        let found = violations(
            "pub fn setup(s: &mut S) { s.spawn((Node,)); }
             #[cfg(test)]
             mod tests {
                 use lele_bevy_preview::run;
                 fn copy(s: &mut S) { s.spawn((Node,)); }
                 fn build(app: &mut App) { let _ = super::setup; app.add_systems(Startup, copy); }
                 #[test]
                 fn spawn_root_ui_png_preview() {
                     let scene = Scene { build, states: vec![] };
                     run(&scene, &Config::default(), \"x\").expect(\"preview\");
                 }
             }",
        );
        assert_eq!(found.len(), 1);
        assert!(found[0].message.contains("not a copy"));
    }
}
