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
        if !scan::ScannedFile(file).defines_plugin() {
            continue;
        }
        let spawns_ui = scan::BevyProject(project).dir_spawns_ui(source.origin, rel_path)
            || scan::BevyProject(project)
                .registers_ui(&scan::ScannedFile(file).plugin_registrations(project));
        if !spawns_ui {
            continue;
        }

        let missing = scan::ScannedFile(file).require_preview(scan::PreviewKind::Scene, || {
            scan::BevyProject(project).violation(source.origin, rel_path, 1,
                checkers::bevy_plugin_scene::BevyPluginScene::CODE,
                String::from(
                    "file defines a `Plugin` whose domain spawns UI but has no `*_ui_scene_preview` test routing through `lele_bevy_preview::run`",
                ),
            )
        });
        if missing.is_some() {
            violations.extend(missing);
            continue;
        }

        let previews = scan::ScannedFile(file).collect_previews();
        let plugins = scan::ScannedFile(file).plugin_types();
        if let Some(preview) =
            scan::Preview::missing_subject(&previews, scan::PreviewKind::Scene, &plugins)
        {
            violations.push(scan::BevyProject(project).violation(source.origin, rel_path, preview.line,
                checkers::bevy_plugin_scene::BevyPluginScene::CODE,
                format!(
                    "`fn {}` does not assemble the plugin: its `Scene::build` must `add_plugins({})`",
                    preview.name,
                    plugins.join(", ")
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
        assert_eq!(
            violations[0].code,
            checkers::bevy_plugin_scene::BevyPluginScene::CODE
        );
    }
}
