use atomic_delegate_macros::atomic_delegate;
use atomic_delegate_macros::atomic_delegates;

use crate::ErrorFormat;
use crate::ExampleFile;
use crate::Location;
use crate::Project;
use crate::Rule;
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
    pub fn render_rule(rule: &dyn Rule) -> String {}
    pub fn print_rule_list(&self) {}
    pub fn print_violations(&self, violations: &[RuleViolation]) {}
}

#[rustfmt::skip]
impl Linter {
    pub fn explain(&self, code_or_name: &str) -> Option<String> {
        self.rules
            .iter()
            .find(|c| c.code().eq_ignore_ascii_case(code_or_name) || c.name() == code_or_name)
            .map(|c| Self::render_rule(c.as_ref()))
    }
    pub(crate) fn print_clippy(d: &RuleViolation, w: &mut impl Write) {
        let _ = match d.location {
            Location::File => writeln!(w, "{}: error[{}]: {}", d.file.display(), d.code, d.message),
            Location::Site { line, col } => writeln!(
                w,
                "{}:{}:{}: error[{}]: {}",
                d.file.display(),
                line,
                col,
                d.code,
                d.message
            ),
        };
    }
    pub(crate) fn print_github(d: &RuleViolation, w: &mut impl Write) {
        let _ = match d.location {
            Location::File => writeln!(
                w,
                "::error file={file},line=1,title={code}::{message}",
                file = d.file.display(),
                code = d.code,
                message = d.message
            ),
            Location::Site { line, col } => writeln!(
                w,
                "::error file={file},line={line},col={col},title={code}::{message}",
                file = d.file.display(),
                code = d.code,
                message = d.message
            ),
        };
    }
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
