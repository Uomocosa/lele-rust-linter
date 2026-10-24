use lele_lint::ErrorFormat;
use lele_lint::Linter;
use lele_lint::Rule;

use crate::bevy_linter::BevyLinter;
use crate::rules;

pub fn from_error_format(error_format: ErrorFormat) -> BevyLinter {
    let mut rules: Vec<Box<dyn Rule>> = Vec::new();
    rules::bevy_export::BevyExport::register(&mut rules);
    rules::bevy_folder::BevyFolder::register(&mut rules);
    rules::bevy_ui::BevyUi::register(&mut rules);
    rules::bevy_ui_mp4::BevyUiMp4::register(&mut rules);
    rules::bevy_ui_restyle::BevyUiRestyle::register(&mut rules);
    rules::bevy_plugin_scene::BevyPluginScene::register(&mut rules);
    rules::preview_routing::PreviewRouting::register(&mut rules);
    rules::preview_centered::PreviewCentered::register(&mut rules);
    BevyLinter(Linter {
        rules,
        error_format,
    })
}

#[cfg(test)]
mod tests {
    use super::from_error_format;
    use lele_lint::ErrorFormat;

    #[test]
    fn test_usage() {
        let linter = from_error_format(ErrorFormat::Clippy);
        assert_eq!(linter.rules.len(), 8);
    }
}
