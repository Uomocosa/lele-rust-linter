use lele_lint::Project;
use lele_lint::RuleViolation;

use crate::checkers;
use crate::scan;

const CENTERED_KINDS: [&str; 2] = ["System", "Component"];

pub fn check(
    _self: &checkers::preview_centered::PreviewCentered,
    project: &Project,
) -> Vec<RuleViolation> {
    let mut violations = Vec::new();
    for source in project.sources() {
        for preview in scan::collect_previews(source.file) {
            for kind in &preview.scene.uncentered_kinds {
                if !CENTERED_KINDS.contains(&kind.as_str()) {
                    continue;
                }
                violations.push(scan::site_violation(
                    project,
                    source.origin,
                    source.relative_path,
                    preview.line,
                    checkers::preview_centered::PreviewCentered::CODE,
                    format!(
                        "`fn {}` previews a `Kind::{kind}` scene without centring it; parent its nodes under `lele_bevy_preview::scene::centered_root()` or chain `lele_bevy_preview::scene::center_roots` after the spawning system",
                        preview.name
                    ),
                ));
            }
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

    fn messages(source: &str) -> Vec<String> {
        let mut project = Project::default();
        project
            .parsed_files
            .insert(PathBuf::from("ui/root.rs"), syn::parse_str(source).unwrap());
        check(&checkers::preview_centered::PreviewCentered, &project)
            .into_iter()
            .map(|violation| violation.message)
            .collect()
    }

    fn preview(kind: &str, build_body: &str) -> String {
        format!(
            "#[cfg(test)]
             mod tests {{
                 fn spawn_rows(mut commands: Commands) {{ commands.spawn((Node::default(),)); }}
                 fn build(app: &mut App) {{ {build_body} }}
                 #[test]
                 fn rows_ui_png_preview() {{
                     let scene = Scene {{ kind: Kind::{kind}, build, states: vec![] }};
                     lele_bevy_preview::run(&scene, &c, \"x\").expect(\"preview\");
                 }}
             }}"
        )
    }

    #[test]
    fn test_usage() {
        let found = messages(&preview("System", "app.add_systems(Startup, spawn_rows);"));
        assert_eq!(found.len(), 1);
        assert!(found[0].contains("Kind::System"));
    }

    #[test]
    fn test_usage_component_is_checked_but_plugin_and_app_are_exempt() {
        assert_eq!(
            messages(&preview(
                "Component",
                "app.add_systems(Startup, spawn_rows);"
            ))
            .len(),
            1
        );
        assert_eq!(
            messages(&preview("Plugin", "app.add_systems(Startup, spawn_rows);")),
            Vec::<String>::new()
        );
        assert_eq!(
            messages(&preview("App", "app.add_systems(Startup, spawn_rows);")),
            Vec::<String>::new()
        );
    }

    #[test]
    fn test_usage_center_roots_in_build_is_clean() {
        let found = messages(&preview(
            "System",
            "app.add_systems(Startup, (spawn_rows, lele_bevy_preview::scene::center_roots).chain());",
        ));
        assert_eq!(found, Vec::<String>::new());
    }
}
