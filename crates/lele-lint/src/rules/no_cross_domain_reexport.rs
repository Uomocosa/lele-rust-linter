use crate::Category;
use crate::ExampleFile;
use crate::Project;
use crate::Rule;
use crate::RuleDoc;
use crate::RuleViolation;
use atomic_delegate_macros::atomic_delegate;
use atomic_delegate_macros::atomic_delegates;
use rule_examples_macro::rule_examples;
use std::path::Path;

pub struct NoCrossDomainReexport;

impl NoCrossDomainReexport {
    pub const NAME: &'static str = "no_cross_domain_reexport";
    pub const CODE: &'static str = "LELE005";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Imports,
        summary: "A `mod.rs` re-exports only items from its own folder; cross-domain re-exports go in `lib.rs`.",
        why: "Each domain's `mod.rs` then describes only that domain, and the crate's public surface is in one file.",
        fix: "Delete the `pub use` of another domain's item from this `mod.rs` and add it to `lib.rs` instead.",
        bad: rule_examples!("no_cross_domain_reexport/bad"),
        good: rule_examples!("no_cross_domain_reexport/good"),
    };
}

#[rustfmt::skip]
impl Rule for NoCrossDomainReexport {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(NoCrossDomainReexport)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl NoCrossDomainReexport {
    pub fn register(rules: &mut Vec<Box<dyn Rule>>) {
        rules.push(Box::new(NoCrossDomainReexport));
    }
    pub(crate) fn own_domain_of(mod_rs_path: &Path) -> String {
        mod_rs_path
            .components()
            .next()
            .map(|component| component.as_os_str().to_string_lossy().to_string())
            .unwrap_or_default()
    }
}

#[atomic_delegates]
impl NoCrossDomainReexport {
    pub(crate) fn reexport_target_domain(segments: &[String]) -> Option<String> {}
}

// no test_usage necessary
