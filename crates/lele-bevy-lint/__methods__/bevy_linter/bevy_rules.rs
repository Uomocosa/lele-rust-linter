use lele_lint::traits::Rule;

use crate::rules;

pub fn bevy_rules() -> Vec<Box<dyn Rule>> {
    let mut rules: Vec<Box<dyn Rule>> = Vec::new();
    rules::bevy_export::BevyExport::register(&mut rules);
    rules::bevy_folder::BevyFolder::register(&mut rules);
    rules::bevy_ui::BevyUi::register(&mut rules);
    rules::bevy_ui_mp4::BevyUiMp4::register(&mut rules);
    rules::bevy_ui_restyle::BevyUiRestyle::register(&mut rules);
    rules::bevy_plugin_scene::BevyPluginScene::register(&mut rules);
    rules::preview_routing::PreviewRouting::register(&mut rules);
    rules::preview_centered::PreviewCentered::register(&mut rules);
    rules::no_serialized_entity::NoSerializedEntity::register(&mut rules);
    rules::no_tests_in_systems::NoTestsInSystems::register(&mut rules);
    rules
}

#[cfg(test)]
mod tests {
    use super::bevy_rules;

    #[test]
    fn test_usage() {
        let rules = bevy_rules();
        assert_eq!(rules.len(), 10);
        assert!(rules.iter().any(|rule| rule.name() == "bevy_ui"));
    }
}
