use crate::rules;
use crate::AllowWhitelistEntry;
use crate::RuleViolation;
use std::path::PathBuf;
use syn::spanned::Spanned;
use syn::visit::Visit;

pub(crate) struct AllowFinder<'a> {
    pub(crate) file: PathBuf,
    pub(crate) crate_rel: PathBuf,
    pub(crate) clippy_allow_whitelist: &'a [AllowWhitelistEntry],
    pub(crate) violations: Vec<RuleViolation>,
}

impl<'ast> Visit<'ast> for AllowFinder<'_> {
    fn visit_attribute(&mut self, attr: &'ast syn::Attribute) {
        let kind = if attr.path().is_ident("allow") {
            "allow"
        } else if attr.path().is_ident("expect") {
            "expect"
        } else {
            return;
        };
        let lint_paths = rules::no_allow_attributes::NoAllowAttributes::lint_paths(attr);
        if self.is_whitelisted(&lint_paths) {
            return;
        }
        let start = attr.span().start();
        self.violations.push(RuleViolation::from_site(self.file.clone(), start.line, start.column, rules::no_allow_attributes::NoAllowAttributes::CODE, format!(
                "`{kind}` attribute is banned — add an [[lele.lint.clippy_allow_whitelist]] entry with the exact `allow` lint path, `file` and a non-empty `reason` in lele.toml to whitelist it"
            )));
    }
}

impl AllowFinder<'_> {
    pub(crate) fn is_whitelisted(&self, lint_paths: &[String]) -> bool {
        if lint_paths.is_empty() {
            return false;
        }
        self.clippy_allow_whitelist.iter().any(|entry| {
            !entry.reason.trim().is_empty()
                && self.crate_rel == entry.file
                && lint_paths.iter().all(|path| path == &entry.allow)
        })
    }
}

// no test_usage necessary
