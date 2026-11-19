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
use std::path::PathBuf;

pub struct TestModuleItems;

impl TestModuleItems {
    pub const NAME: &'static str = "test_module_items";
    pub const CODE: &'static str = "LELETEST005";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Tests,
        summary: "A `#[cfg(test)]` module holds only test fns (`#[test]`, `#[rstest]`, `#[tokio::test]`), `use` and `const`.",
        why: "Every other item is a test helper; it belongs in the crate's `src/testing/` (LELETEST006) so the next test reuses it instead of copying it (RATIONALE §17, SMELLS S9).",
        fix: "Move the item to `src/testing/__fixtures__/` (an app builder → `__app__/`, a world query → `__queries__/`, a fake → `__fakes__/`) and import it, or delete it if the test no longer needs it.",
        bad: rule_examples!("test_module_items/bad"),
        good: rule_examples!("test_module_items/good"),
    };
}

#[rustfmt::skip]
impl Rule for TestModuleItems {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(TestModuleItems, Rule)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl TestModuleItems {
    pub fn register(rules: &mut Vec<Box<dyn Rule>>) {
        rules.push(Box::new(TestModuleItems));
    }
}

#[atomic_delegates]
impl TestModuleItems {
    pub(crate) fn is_test_module(item: &syn::Item) -> bool {}
    pub(crate) fn is_test_fn(function: &syn::ItemFn) -> bool {}
    pub(crate) fn is_allowed(item: &syn::Item) -> bool {}
    pub(crate) fn is_testing_fixture_root(item: &syn::Item) -> bool {}
    pub(crate) fn out_of_line_test_files(rel_path: &Path, file: &syn::File) -> Vec<PathBuf> {}
}

// no test_usage necessary
