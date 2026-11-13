use crate::traits::Rule;
use crate::Category;
use crate::ExampleFile;
use crate::Project;
use crate::RuleDoc;
use crate::RuleViolation;
use atomic_delegate_macros::atomic_delegate;
use atomic_delegate_macros::atomic_delegates;
use rule_examples_macro::rule_examples;
use std::path::Path;

pub struct NoStutteredType;

impl NoStutteredType {
    pub const NAME: &'static str = "no_stuttered_type";
    pub const CODE: &'static str = "LELE012";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Types,
        summary: "A type does not repeat its folder's name (`freenet::Client`, not `freenet::FreenetClient`).",
        why: "Code always says `freenet::Client` through the domain prefix, so the prefix in the type name is said twice.",
        fix: "Drop the folder name from the type name (`freenet::FreenetClient` → `freenet::Client`) and update every use.",
        bad: rule_examples!("no_stuttered_type/bad"),
        good: rule_examples!("no_stuttered_type/good"),
    };
}

#[rustfmt::skip]
impl Rule for NoStutteredType {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(NoStutteredType, Rule)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl NoStutteredType {
    pub fn register(rules: &mut Vec<Box<dyn Rule>>) {
        rules.push(Box::new(NoStutteredType));
    }
}

#[atomic_delegates]
impl NoStutteredType {
    pub(crate) fn parent_dir_name(rel_path: &Path) -> Option<String> {}
    pub(crate) fn primary_exposed_type(file: &syn::File, stem: &str) -> Option<String> {}
    pub(crate) fn strip_dir_prefix(snake: &str, dir: &str) -> Option<String> {}
}

// no test_usage necessary
