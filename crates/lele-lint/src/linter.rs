use atomic_delegate_macros::atomic_delegate;
use atomic_delegate_macros::atomic_delegates;

use crate::traits::Rule;
use crate::ErrorFormat;
use crate::ExampleFile;
use crate::Project;
use crate::RuleViolation;
use std::fmt::Write as _;
use std::io::Write;

pub struct Linter {
    pub rules: Vec<Box<dyn Rule>>,
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
    pub fn skill_markdown(&self) -> String {}
    pub fn render_rule(rule: &dyn Rule) -> String {}
    pub fn print_rule_list(&self) {}
    pub fn print_violations(&self, violations: &[RuleViolation]) {}
    pub fn retain_codes(&mut self, codes: &[String]) -> Vec<String> {}
    pub fn render_report(
        violations: &[RuleViolation],
        rules: &[Box<dyn Rule>],
        color: bool,
    ) -> String {
    }
    pub fn render_violation(
        violation: &RuleViolation,
        source: Option<&str>,
        rule_fix: Option<&str>,
        color: bool,
    ) -> String {
    }
    pub fn render_summary(violations: &[RuleViolation], color: bool) -> String {}
    pub fn explain(&self, code_or_name: &str) -> Option<String> {}
}

#[atomic_delegates]
impl Linter {
    pub(crate) fn use_color() -> bool {}
    pub(crate) fn print_github(d: &RuleViolation, w: &mut impl Write) {}
}

#[rustfmt::skip]
impl Linter {
    pub(crate) fn paint(color: bool, style: &str, text: &str) -> String {
        if color { format!("\x1b[{style}m{text}\x1b[0m") } else { text.to_owned() }
    }
}

impl Linter {
    pub(crate) fn render_files(files: &[ExampleFile]) -> String {
        let mut out = String::new();
        for file in files {
            let lang = if file.path.ends_with(".toml") {
                "toml"
            } else {
                "rust"
            };
            let _ = writeln!(out, "`{}`\n", file.path);
            let _ = writeln!(out, "```{lang}\n{}```\n", file.source);
        }
        out
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
            .rules
            .iter()
            .find(|rule| rule.name() == "no_positional")
            .map(|rule| rule.code().to_lowercase());
        let by_code = linter.explain(code.as_deref().unwrap_or(""));
        assert!(by_code.is_some());
        assert_eq!(by_code, by_name);
        assert!(linter.explain("LELE999").is_none());
    }
}
