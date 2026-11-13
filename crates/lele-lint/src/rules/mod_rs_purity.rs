use crate::traits::Rule;
use crate::Category;
use crate::ExampleFile;
use crate::Project;
use crate::RuleDoc;
use crate::RuleViolation;
use atomic_delegate_macros::atomic_delegate;
use atomic_delegate_macros::atomic_delegates;
use rule_examples_macro::rule_examples;

pub struct ModRsPurity;

impl ModRsPurity {
    pub const NAME: &'static str = "mod_rs_purity";
    pub const CODE: &'static str = "LELE023";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Layout,
        summary: "`mod.rs` holds only `mod`/`pub mod` declarations and `pub use` re-exports.",
        why: "`mod.rs` is the folder's table of contents; logic hidden there is logic nobody looks for.",
        fix: "Move every item that is not `mod`, `pub mod` or `pub use` out of `mod.rs` into its own atomic file, and re-export it from `mod.rs`.",
        bad: rule_examples!("mod_rs_purity/bad"),
        good: rule_examples!("mod_rs_purity/good"),
    };
}

#[rustfmt::skip]
impl Rule for ModRsPurity {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(ModRsPurity, Rule)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl ModRsPurity {
    pub fn register(rules: &mut Vec<Box<dyn Rule>>) {
        rules.push(Box::new(ModRsPurity));
    }
}

#[atomic_delegates]
impl ModRsPurity {
    pub(crate) fn violation_message(item: &syn::Item) -> Option<String> {}
    pub(crate) fn named_message(kind: &str, name: &str, vis: &syn::Visibility) -> String {}
}

// no test_usage necessary
