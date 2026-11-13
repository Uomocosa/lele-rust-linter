use crate::AllowWhitelistEntry;
use crate::Project;
use crate::RuleViolation;
use atomic_delegate_macros::atomic_delegates;
use std::path::PathBuf;
use syn::visit::Visit;

pub(crate) struct AllowFinder<'a> {
    pub(crate) file: PathBuf,
    pub(crate) config_base: PathBuf,
    pub(crate) clippy_allow_whitelist: &'a [AllowWhitelistEntry],
    pub(crate) violations: Vec<RuleViolation>,
}

#[atomic_delegates]
impl<'ast> Visit<'ast> for AllowFinder<'_> {
    fn visit_attribute(&mut self, attr: &'ast syn::Attribute) {}
}

impl AllowFinder<'_> {
    pub(crate) fn is_whitelisted(&self, lint_paths: &[String]) -> bool {
        if lint_paths.is_empty() {
            return false;
        }
        self.clippy_allow_whitelist.iter().any(|entry| {
            !entry.reason.trim().is_empty()
                && Project::same_file(&self.config_base.join(&entry.file), &self.file)
                && lint_paths.iter().all(|path| path == &entry.allow)
        })
    }
}

// no test_usage necessary
