use lele_lint::Project;
use lele_lint::RuleViolation;

use crate::checkers;
use crate::scan;

pub fn check(
    _self: &checkers::bevy_plugin_scene::BevyPluginScene,
    project: &Project,
) -> Vec<RuleViolation> {
    let mut violations = Vec::new();

    for source in project.sources() {
        let rel_path = source.relative_path;
        let file = source.file;
        if !scan::defines_plugin(file) || !scan::dir_spawns_ui(project, source.origin, rel_path) {
            continue;
        }

        violations.extend(scan::require_preview(
            project,
            source.origin,
            rel_path,
            file,
            scan::PreviewKind::Scene,
            checkers::bevy_plugin_scene::BevyPluginScene::CODE,
            || {
                scan::site_violation(
                    project,
                    source.origin,
                    rel_path,
                    1,
                    checkers::bevy_plugin_scene::BevyPluginScene::CODE,
                    String::from(
                        "file defines a `Plugin` whose domain spawns UI but has no ignored `*_ui_scene_preview` test routing through `lele_bevy_preview::run`",
                    ),
                )
            },
        ));
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
            PathBuf::from("ui/spawn.rs"),
            syn::parse_str("pub fn spawn_ui(s: &mut S) { s.spawn((Node,)); }").unwrap(),
        );
        project.parsed_files.insert(
            PathBuf::from("ui/plugin.rs"),
            syn::parse_str(
                "pub struct MyPlugin;
                 impl Plugin for MyPlugin { fn build(&self, app: &mut A) { } }",
            )
            .unwrap(),
        );
        let violations = check(&checkers::bevy_plugin_scene::BevyPluginScene, &project);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].code, "E038");
    }
}
