use crate::Category;
use crate::ExampleFile;
use crate::ImplMethod;
use crate::Project;
use crate::Rule;
use crate::RuleDoc;
use crate::RuleViolation;
use atomic_delegate_macros::atomic_delegate;
use atomic_delegate_macros::atomic_delegates;
use rule_examples_macro::rule_examples;

pub struct ConstructorNoSkip;

impl ConstructorNoSkip {
    pub const NAME: &'static str = "constructor_no_skip";
    pub const CODE: &'static str = "LELE018";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Delegates,
        summary: "`impl Default` and real constructors are never `#[rustfmt::skip]`.",
        why: "Constructors list every field; rustfmt keeps them one field per line so diffs stay readable.",
        bad: rule_examples!("constructor_no_skip/bad"),
        good: rule_examples!("constructor_no_skip/good"),
    };
}

#[rustfmt::skip]
impl Rule for ConstructorNoSkip {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(ConstructorNoSkip)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl ConstructorNoSkip {
    pub fn register(rules: &mut Vec<Box<dyn Rule>>) {
        rules.push(Box::new(ConstructorNoSkip));
    }
    pub(crate) fn has_any_real_constructor(impl_block: &syn::ItemImpl) -> bool {
        impl_block.items.iter().any(
            |item| matches!(item, syn::ImplItem::Fn(method) if ImplMethod(method).is_real_constructor()),
        )
    }
}

#[atomic_delegates]
impl ConstructorNoSkip {
    pub(crate) fn type_name_string(ty: &syn::Type) -> String {}
    pub(crate) fn is_atomic_delegates(impl_block: &syn::ItemImpl) -> bool {}
}

// no test_usage necessary
