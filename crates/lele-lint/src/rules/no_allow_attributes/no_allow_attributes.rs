use crate::traits::Rule;
use crate::Category;
use crate::ExampleFile;
use crate::Project;
use crate::RuleDoc;
use crate::RuleViolation;
use atomic_delegate_macros::atomic_delegate;
use atomic_delegate_macros::atomic_delegates;
use rule_examples_macro::rule_examples;

pub struct NoAllowAttributes;

impl NoAllowAttributes {
    pub const NAME: &'static str = "no_allow_attributes";
    pub const CODE: &'static str = "LELE024";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Style,
        summary: "No `#[allow(...)]`/`#[expect(...)]` unless whitelisted in `lele.toml` with the exact lint, file and a reason (the file is relative to that lele.toml).",
        why: "Every silenced lint is a decision the user made on purpose, recorded with its reason.",
        fix: "Remove the `#[allow]`/`#[expect]` and fix the code the lint points at; a whitelist entry in `lele.toml` is the user's decision, never yours.",
        bad: rule_examples!("no_allow_attributes/bad"),
        good: rule_examples!("no_allow_attributes/good"),
    };
}

#[rustfmt::skip]
impl Rule for NoAllowAttributes {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(NoAllowAttributes)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl NoAllowAttributes {
    pub fn register(rules: &mut Vec<Box<dyn Rule>>) {
        rules.push(Box::new(NoAllowAttributes));
    }
}

#[atomic_delegates]
impl NoAllowAttributes {
    pub(crate) fn lint_paths(attr: &syn::Attribute) -> Vec<String> {}
}

// no test_usage necessary
