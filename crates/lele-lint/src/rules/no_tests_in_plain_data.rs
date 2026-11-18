use crate::traits::Rule;
use crate::Category;
use crate::ExampleFile;
use crate::Project;
use crate::RuleDoc;
use crate::RuleViolation;
use atomic_delegate_macros::atomic_delegate;
use rule_examples_macro::rule_examples;

pub struct NoTestsInPlainData;

impl NoTestsInPlainData {
    pub const NAME: &'static str = "no_tests_in_plain_data";
    pub const CODE: &'static str = "LELETEST004";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Tests,
        summary: "No `#[cfg(test)]` in plain-data files (plain structs/enums, pure `mod.rs`/`lib.rs`, `constants.rs`).",
        why: "Plain data has no behaviour of its own; it is tested through the code that uses it. Tuning values live in constants.rs and are not asserted (D8).",
        fix: "Delete the `#[cfg(test)]` block from the plain-data file and test the method or function that uses the type instead.",
        bad: rule_examples!("no_tests_in_plain_data/bad"),
        good: rule_examples!("no_tests_in_plain_data/good"),
    };
}

#[rustfmt::skip]
impl Rule for NoTestsInPlainData {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(NoTestsInPlainData, Rule)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl NoTestsInPlainData {
    pub fn register(rules: &mut Vec<Box<dyn Rule>>) {
        rules.push(Box::new(NoTestsInPlainData));
    }
}

// no test_usage necessary
