use crate::rules;
use crate::traits::Rule;
use crate::Category;
use crate::ExampleFile;
use crate::Origin;
use crate::Project;
use crate::RuleDoc;
use crate::RuleViolation;
use atomic_delegate_macros::atomic_delegate;
use atomic_delegate_macros::atomic_delegates;
use rule_examples_macro::rule_examples;
use std::path::Path;

#[path = "test_usage_shape/__basic__/mod.rs"]
mod basic;
mod test_usage_calls;
pub(crate) use basic::enums::PrimaryKind;
pub(crate) use basic::structs::PrimaryItem;
pub(crate) use test_usage_calls::TestUsageCalls;

pub struct TestUsageShape;

impl TestUsageShape {
    pub const NAME: &'static str = "test_usage_shape";
    pub const CODE: &'static str = "LELECORE007";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Tests,
        summary: "A file has exactly one `test_usage`; it calls the file's primary item and asserts on a value that call produced.",
        why: "One usage test per file is the file's documentation (D1); a `test_usage` that never calls the item, or asserts nothing, proves nothing (D4).",
        fix: "Keep one `test_usage`: construct the type / call the method / add the plugin, bind the result and `assert*!` on it (or on its receiver/arguments when it returns `()`). Rename the other tests for their corner case; `#[rstest]` for honest ones.",
        bad: rule_examples!("test_usage_shape/bad"),
        good: rule_examples!("test_usage_shape/good"),
    };
}

#[rustfmt::skip]
impl Rule for TestUsageShape {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(TestUsageShape, Rule)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl TestUsageShape {
    pub fn register(rules: &mut Vec<Box<dyn Rule>>) {
        rules.push(Box::new(TestUsageShape));
    }
}

#[atomic_delegates]
impl TestUsageShape {
    pub(crate) fn test_usage_fns(file: &syn::File) -> Vec<&syn::ItemFn> {}
    pub(crate) fn primary_item(
        file: &syn::File,
        rel_path: &Path,
        origin: Origin,
    ) -> Option<rules::test_usage_shape::PrimaryItem> {
    }
    pub(crate) fn calls_primary(
        calls: &rules::test_usage_shape::TestUsageCalls,
        primary: &rules::test_usage_shape::PrimaryItem,
    ) -> bool {
    }
    pub(crate) fn asserts_derived_value(
        usage: &syn::ItemFn,
        calls: &rules::test_usage_shape::TestUsageCalls,
        primary: &rules::test_usage_shape::PrimaryItem,
    ) -> bool {
    }
}

// no test_usage necessary
