use lele_lint::Project;
use lele_lint::RuleViolation;

use crate::rules;
use crate::scan;

pub fn check(
    _self: &rules::bevy_ui_restyle::BevyUiRestyle,
    project: &Project,
) -> Vec<RuleViolation> {
    let graph = scan::CallGraph::from(project);
    let mut violations = Vec::new();

    for source in project.sources() {
        let rel_path = source.relative_path;
        let file = source.file;
        let restyled = scan::ScannedFile(file).restyled_systems(&graph);
        let Some(first) = restyled.first() else {
            continue;
        };
        let mut subjects: Vec<String> = restyled
            .iter()
            .map(|restyle| restyle.system.clone())
            .collect();
        subjects.extend(scan::ScannedFile(file).plugin_types());
        subjects.sort();
        subjects.dedup();
        let subject_list = subjects.join("`, `");

        let kind = if scan::ScannedFile(file).time_drivers().is_empty() {
            scan::PreviewKind::Png
        } else {
            scan::PreviewKind::Mp4
        };
        let previews = scan::ScannedFile(file).collect_previews();
        let rendering = previews
            .iter()
            .filter(|preview| preview.kind == kind)
            .find(|preview| {
                preview
                    .scene
                    .build_uses
                    .iter()
                    .any(|used| subjects.contains(used))
            });

        let Some(preview) = rendering else {
            violations.push(scan::BevyProject(project).violation(source.origin, rel_path, first.line,
                rules::bevy_ui_restyle::BevyUiRestyle::CODE,
                format!(
                    "`fn {}` restyles UI on `{}` but defines no `*_ui_{}_preview` test whose `Scene::build` renders it (register, call or add one of `{}`)",
                    first.system,
                    subject_list,
                    rules::bevy_ui_restyle::BevyUiRestyle::suffix(kind),
                    subject_list,
                ),
            ));
            continue;
        };

        if kind == scan::PreviewKind::Png && preview.scene.state_count < 2 {
            violations.push(scan::BevyProject(project).violation(source.origin, rel_path, preview.line,
                rules::bevy_ui_restyle::BevyUiRestyle::CODE,
                format!(
                    "`fn {}` shows one state; a restyle needs at least two `State`s (before, after)",
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
    use crate::rules;
    use lele_lint::Project;
    use std::path::PathBuf;

    fn violations(source: &str) -> Vec<lele_lint::RuleViolation> {
        let mut project = Project::default();
        project
            .parsed_files
            .insert(PathBuf::from("ui/gate.rs"), syn::parse_str(source).unwrap());
        check(&rules::bevy_ui_restyle::BevyUiRestyle, &project)
    }

    #[test]
    fn test_usage() {
        let found = violations(
            "pub fn gate(button: Query<&mut BackgroundColor>) { let _ = button; }
             pub fn build(app: &mut App) { app.add_systems(Update, gate); }",
        );
        assert_eq!(found.len(), 1);
        assert_eq!(found[0].code, rules::bevy_ui_restyle::BevyUiRestyle::CODE);
    }

    #[test]
    fn test_usage_two_states_is_clean() {
        let found = violations(
            "pub fn gate(button: Query<&mut BackgroundColor>) { let _ = button; }
             pub fn build(app: &mut App) { app.add_systems(Update, gate); }
             #[cfg(test)]
             mod tests {
                 use lele_bevy_preview::run;
                 use super::gate;
                 fn preview_build(app: &mut App) { app.add_systems(Update, gate); }
                 fn rest(_world: &mut World) {}
                 #[test]
                 fn gate_ui_png_preview() {
                     let scene = Scene { build: preview_build, states: vec![State { apply: rest }, State { apply: rest }] };
                     run(&scene, &Config::default(), \"x\").expect(\"preview\");
                 }
             }",
        );
        assert!(found.is_empty());
    }

    #[test]
    fn test_usage_one_state_is_flagged() {
        let found = violations(
            "pub fn gate(button: Query<&mut BackgroundColor>) { let _ = button; }
             pub fn build(app: &mut App) { app.add_systems(Update, gate); }
             #[cfg(test)]
             mod tests {
                 use lele_bevy_preview::run;
                 use super::gate;
                 fn preview_build(app: &mut App) { app.add_systems(Update, gate); }
                 fn rest(_world: &mut World) {}
                 #[test]
                 fn gate_ui_png_preview() {
                     let scene = Scene { build: preview_build, states: vec![State { apply: rest }] };
                     run(&scene, &Config::default(), \"x\").expect(\"preview\");
                 }
             }",
        );
        assert_eq!(found.len(), 1);
        assert!(found[0].message.contains("two `State`s"));
    }

    #[test]
    fn test_usage_time_driven_needs_mp4() {
        let found = violations(
            "pub fn gate(time: Res<Time>, button: Query<&mut BackgroundColor>) { let _ = (time.delta_secs(), button); }
             pub fn build(app: &mut App) { app.add_systems(Update, gate); }
             #[cfg(test)]
             mod tests {
                 use lele_bevy_preview::run;
                 use super::gate;
                 fn preview_build(app: &mut App) { app.add_systems(Update, gate); }
                 fn rest(_world: &mut World) {}
                 #[test]
                 fn gate_ui_png_preview() {
                     let scene = Scene { build: preview_build, states: vec![State { apply: rest }, State { apply: rest }] };
                     run(&scene, &Config::default(), \"x\").expect(\"preview\");
                 }
             }",
        );
        assert_eq!(found.len(), 1);
        assert!(found[0].message.contains("mp4"));
    }
}
