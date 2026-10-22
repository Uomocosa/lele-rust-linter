use lele_lint::ErrorFormat;

use crate::bevy_linter::BevyLinter;

pub fn default() -> BevyLinter {
    let error_format = ErrorFormat::Clippy;
    BevyLinter::with_error_format(error_format)
}

#[cfg(test)]
mod tests {
    use super::default;

    #[test]
    fn test_usage() {
        assert_eq!(default().checkers.len(), 8);
    }
}
