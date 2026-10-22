use lele_lint::Checker;
use lele_lint::ErrorFormat;
use lele_lint::Linter;

use crate::bevy_linter::BevyLinter;
use crate::checkers;

pub fn from_error_format(error_format: ErrorFormat) -> BevyLinter {
    let mut checkers: Vec<Box<dyn Checker>> = Vec::new();
    checkers::bevy_export::BevyExport::register(&mut checkers);
    checkers::bevy_folder::BevyFolder::register(&mut checkers);
    checkers::bevy_ui::BevyUi::register(&mut checkers);
    checkers::bevy_ui_mp4::BevyUiMp4::register(&mut checkers);
    checkers::bevy_ui_restyle::BevyUiRestyle::register(&mut checkers);
    checkers::bevy_plugin_scene::BevyPluginScene::register(&mut checkers);
    checkers::preview_routing::PreviewRouting::register(&mut checkers);
    checkers::preview_centered::PreviewCentered::register(&mut checkers);
    BevyLinter(Linter {
        checkers,
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
        assert_eq!(linter.checkers.len(), 8);
    }
}
