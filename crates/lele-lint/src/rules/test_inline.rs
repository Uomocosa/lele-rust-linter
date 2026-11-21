use crate::traits::Rule;
use crate::Category;
use crate::ExampleFile;
use crate::Project;
use crate::RuleDoc;
use crate::RuleViolation;
use atomic_delegate_macros::atomic_delegate;
use atomic_delegate_macros::atomic_delegates;
use rule_examples_macro::rule_examples;

pub struct TestInline;

impl TestInline {
    pub const NAME: &'static str = "test_inline";
    pub const CODE: &'static str = "LELECORE004";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Tests,
        summary: "Unit tests live in the same file as the code (no `tests/` folders under `src/`, no `*_tests.rs` files, no unlisted crate-level `tests/` dirs).",
        why: "The test sits next to what it tests, so reading one means reading the other. Only listed end-to-end suites may live outside the crate.",
        fix: "Move the test into a `#[cfg(test)] mod tests` at the bottom of the file it tests and delete the separate file or folder. A crate-level `tests/` dir holding integration/e2e tests must be listed in `[lele.project] e2e_test_dirs` (paths relative to the config file); unit tests go colocated.",
        bad: rule_examples!("test_inline/bad"),
        good: rule_examples!("test_inline/good"),
    };
}

#[rustfmt::skip]
impl Rule for TestInline {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(TestInline, Rule)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl TestInline {
    pub fn register(rules: &mut Vec<Box<dyn Rule>>) {
        rules.push(Box::new(TestInline));
    }
}

#[atomic_delegates]
impl TestInline {
    pub(crate) fn has_tests_component(project: &Project, path: &std::path::Path) -> bool {}
    pub(crate) fn has_underscore_tests_stem(path: &std::path::Path) -> bool {}
    pub(crate) fn file_has_test_items(file: &syn::File) -> bool {}
    pub(crate) fn is_listed_e2e_dir(project: &Project, dir: &std::path::Path) -> bool {}
}

// no test_usage necessary
