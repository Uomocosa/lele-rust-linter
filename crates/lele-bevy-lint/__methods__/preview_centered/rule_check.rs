use lele_lint::Project;
use lele_lint::RuleViolation;

use crate::rules;
use crate::scan;

const CENTERED_KINDS: [&str; 2] = ["System", "Component"];

pub fn rule_check(
    _self: &rules::preview_centered::PreviewCentered,
    project: &Project,
) -> Vec<RuleViolation> {
    let mut violations = Vec::new();
    for source in project.sources() {
        for preview in scan::ScannedFile(source.file).collect_previews() {
            for kind in &preview.scene.uncentered_kinds {
                if !CENTERED_KINDS.contains(&kind.as_str()) {
                    continue;
                }
                violations.push(scan::BevyProject(project).violation(source.origin, source.relative_path, preview.line,
                    rules::preview_centered::PreviewCentered::CODE,
                    format!(
                        "`fn {}` previews a `Kind::{kind}` scene without centring it; parent its nodes under `lele_bevy_preview::scene::centered_root()` or chain `lele_bevy_preview::scene::bevy_systems::center_roots` after the spawning system",
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
    use crate::rules;
    use crate::testing;

    #[test]
    fn test_usage() {
        let found = testing::messages_of(
            &rules::preview_centered::PreviewCentered,
            "ui/root.rs",
            &testing::preview_source("System", "app.add_systems(Startup, spawn_rows);"),
        );
        assert_eq!(found.len(), 1);
        assert!(found[0].contains("Kind::System"));
    }

    #[test]
    fn test_usage_component_is_checked_but_plugin_and_app_are_exempt() {
        assert_eq!(
            testing::messages_of(
                &rules::preview_centered::PreviewCentered,
                "ui/root.rs",
                &testing::preview_source("Component", "app.add_systems(Startup, spawn_rows);")
            )
            .len(),
            1
        );
        assert_eq!(
            testing::messages_of(
                &rules::preview_centered::PreviewCentered,
                "ui/root.rs",
                &testing::preview_source("Plugin", "app.add_systems(Startup, spawn_rows);")
            ),
            Vec::<String>::new()
        );
        assert_eq!(
            testing::messages_of(
                &rules::preview_centered::PreviewCentered,
                "ui/root.rs",
                &testing::preview_source("App", "app.add_systems(Startup, spawn_rows);")
            ),
            Vec::<String>::new()
        );
    }

    #[test]
    fn test_usage_center_roots_in_build_is_clean() {
        let found = testing::messages_of(
            &rules::preview_centered::PreviewCentered,
            "ui/root.rs",
            &testing::preview_source(
                "System",
                "app.add_systems(Startup, (spawn_rows, lele_bevy_preview::scene::bevy_systems::center_roots).chain());",
            ),
        );
        assert_eq!(found, Vec::<String>::new());
    }
}
