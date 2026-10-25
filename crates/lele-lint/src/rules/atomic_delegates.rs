use crate::Category;
use crate::ExampleFile;
use crate::Project;
use crate::Rule;
use crate::RuleDoc;
use crate::RuleViolation;
use atomic_delegate_macros::atomic_delegate;
use atomic_delegate_macros::atomic_delegates;
use rule_examples_macro::rule_examples;

pub struct AtomicDelegates;

impl AtomicDelegates {
    pub const NAME: &'static str = "atomic_delegates";
    pub const CODE: &'static str = "LELE017";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Delegates,
        summary: "Hand-written methods have at most 3 statements, and an impl block with one-line methods carries `#[rustfmt::skip]`.",
        why: "Longer methods belong in `methods/<type>/<method>.rs`; short ones stay readable as one line each.",
        bad: rule_examples!("atomic_delegates/bad"),
        good: rule_examples!("atomic_delegates/good"),
    };
}

#[rustfmt::skip]
impl Rule for AtomicDelegates {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(AtomicDelegates)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl AtomicDelegates {
    pub fn register(rules: &mut Vec<Box<dyn Rule>>) {
        rules.push(Box::new(AtomicDelegates));
    }
    pub(crate) fn has_any_method(impl_block: &syn::ItemImpl) -> bool {
        impl_block
            .items
            .iter()
            .any(|item| matches!(item, syn::ImplItem::Fn(_)))
    }
    pub(crate) fn names_str(idents: &[&syn::Ident]) -> String {
        idents
            .iter()
            .map(|i| i.to_string())
            .collect::<Vec<_>>()
            .join(", ")
    }
}

#[atomic_delegates]
impl AtomicDelegates {
    pub(crate) fn is_one_line_body(method: &syn::ImplItemFn) -> bool {}
}

// no test_usage necessary
