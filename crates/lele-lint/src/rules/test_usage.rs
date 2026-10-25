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

const OPT_OUT: &str = "// no test_usage necessary";

pub struct TestUsage;

impl TestUsage {
    pub const NAME: &'static str = "test_usage";
    pub const CODE: &'static str = "LELE009";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Tests,
        summary: "Every non-trivial file has a `#[cfg(test)] mod tests` with a `test_usage` test, or ends with `// no test_usage necessary`.",
        why: "The usage test is the function's documentation: it shows how to call it and proves it works.",
        bad: rule_examples!("test_usage/bad"),
        good: rule_examples!("test_usage/good"),
    };
}

#[rustfmt::skip]
impl Rule for TestUsage {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(TestUsage)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl TestUsage {
    pub fn register(rules: &mut Vec<Box<dyn Rule>>) {
        rules.push(Box::new(TestUsage));
    }
    pub(crate) fn opt_out_at_end(content: &str) -> bool {
        content
            .lines()
            .rev()
            .find(|line| !line.trim().is_empty())
            .is_some_and(|line| line.trim().starts_with(OPT_OUT))
    }
}

#[atomic_delegates]
impl TestUsage {
    pub(crate) fn reads_opt_out(path: &Path) -> bool {}
    pub(crate) fn is_exempt(rel_path: &Path, file: &syn::File, project: &Project) -> bool {}
    pub(crate) fn is_pure_module_tree(file: &syn::File) -> bool {}
    pub(crate) fn is_type_only(file: &syn::File) -> bool {}
    pub(crate) fn is_default_only_impl(impl_block: &syn::ItemImpl) -> bool {}
    pub(crate) fn is_atomic_delegate_only(file: &syn::File) -> bool {}
    pub(crate) fn is_likely_delegate_impl(impl_block: &syn::ItemImpl) -> bool {}
}

// no test_usage necessary
