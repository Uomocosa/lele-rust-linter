use atomic_delegate_macros::atomic_delegate;
use atomic_delegate_macros::atomic_delegates;
use derive_more::Deref;
use derive_more::DerefMut;
use lele_lint::ErrorFormat;
use lele_lint::Linter;

#[derive(Deref, DerefMut)]
pub struct BevyLinter(pub Linter);

impl Default for BevyLinter {
    #[atomic_delegate(BevyLinter)]
    fn default() -> Self {}
}

#[atomic_delegates]
impl From<ErrorFormat> for BevyLinter {
    fn from(error_format: ErrorFormat) -> Self {}
}

#[atomic_delegates]
impl BevyLinter {
    pub fn skill_markdown(&self) -> String {}
}

#[cfg(test)]
mod tests {
    use super::BevyLinter;
    use lele_lint::ErrorFormat;

    #[test]
    fn test_usage() {
        let linter = BevyLinter::default();
        assert!(linter.rules.iter().any(|rule| rule.name() == "bevy_ui"));
        let markdown = BevyLinter::from(ErrorFormat::Clippy).skill_markdown();
        assert_ne!(markdown, "");
    }
}
