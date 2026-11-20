use lele_lint::ErrorFormat;
use lele_lint::Linter;

use crate::bevy_linter::BevyLinter;

pub fn from_error_format(error_format: ErrorFormat) -> BevyLinter {
    BevyLinter(Linter {
        rules: BevyLinter::bevy_rules(),
        error_format,
        exemptions: Vec::new(),
    })
}

#[cfg(test)]
mod tests {
    use super::from_error_format;
    use lele_lint::ErrorFormat;

    #[test]
    fn test_usage() {
        let linter = from_error_format(ErrorFormat::Clippy);
        assert_eq!(linter.rules.len(), 10);
    }
}
