use atomic_delegate_macros::atomic_delegate;
use atomic_delegate_macros::atomic_delegates;

use crate::Checker;
use crate::ErrorFormat;
use crate::Project;
use crate::RuleViolation;

pub struct Linter {
    pub checkers: Vec<Box<dyn Checker>>,
    pub error_format: ErrorFormat,
}

impl Default for Linter {
    #[atomic_delegate(Linter)]
    fn default() -> Self {}
}

#[atomic_delegates]
impl Linter {
    pub fn run(&self, project: &Project) -> i32 {}
    pub fn rules_markdown(&self) -> String {}
    pub fn render_rule(checker: &dyn Checker) -> String {}
    pub fn print_checker_list(&self) {}
    pub fn print_violations(&self, violations: &[RuleViolation]) {}
}

#[rustfmt::skip]
impl Linter {
    pub fn explain(&self, code_or_name: &str) -> Option<String> {
        self.checkers
            .iter()
            .find(|c| c.code().eq_ignore_ascii_case(code_or_name) || c.name() == code_or_name)
            .map(|c| Self::render_rule(c.as_ref()))
    }
}

#[cfg(test)]
mod tests {
    use crate::Linter;

    #[test]
    fn test_usage() {
        let linter = Linter::default();
        let by_name = linter.explain("no_positional");
        let code = linter
            .checkers
            .iter()
            .find(|checker| checker.name() == "no_positional")
            .map(|checker| checker.code().to_lowercase());
        let by_code = linter.explain(code.as_deref().unwrap_or(""));
        assert!(by_code.is_some());
        assert_eq!(by_code, by_name);
        assert!(linter.explain("LELE999").is_none());
    }
}
