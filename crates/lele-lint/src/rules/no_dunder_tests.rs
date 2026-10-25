use crate::Category;
use crate::ExampleFile;
use crate::ItemMarker;
use crate::Project;
use crate::Rule;
use crate::RuleDoc;
use crate::RuleViolation;
use atomic_delegate_macros::atomic_delegate;
use rule_examples_macro::rule_examples;
use syn::spanned::Spanned;

pub struct NoDunderTests;

impl NoDunderTests {
    pub const NAME: &'static str = "no_dunder_tests";
    pub const CODE: &'static str = "LELE006";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Tests,
        summary: "No `#[cfg(test)]` inside `__basic__/` containers.",
        why: "Containers hold declarations only; test the function or method that uses the types instead.",
        fix: "Delete the `#[cfg(test)]` block from the `__basic__/` file and test the method or function that uses the type instead.",
        bad: rule_examples!("no_dunder_tests/bad"),
        good: rule_examples!("no_dunder_tests/good"),
    };
}

#[rustfmt::skip]
impl Rule for NoDunderTests {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(NoDunderTests)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl NoDunderTests {
    pub fn register(rules: &mut Vec<Box<dyn Rule>>) {
        rules.push(Box::new(NoDunderTests));
    }
    pub(crate) fn cfg_test_line(item: &syn::Item) -> Option<usize> {
        match item {
            syn::Item::Mod(module)
                if module
                    .attrs
                    .iter()
                    .any(|attr| ItemMarker(attr).is_cfg_test()) =>
            {
                Some(module.span().start().line)
            }
            syn::Item::Fn(function)
                if function
                    .attrs
                    .iter()
                    .any(|attr| ItemMarker(attr).is_cfg_test()) =>
            {
                Some(function.span().start().line)
            }
            _ => None,
        }
    }
}

// no test_usage necessary
