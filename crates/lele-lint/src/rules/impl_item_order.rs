use crate::Category;
use crate::ExampleFile;
use crate::Project;
use crate::Rule;
use crate::RuleDoc;
use crate::RuleViolation;
use atomic_delegate_macros::atomic_delegate;
use rule_examples_macro::rule_examples;

pub struct ImplItemOrder;

impl ImplItemOrder {
    pub const NAME: &'static str = "impl_item_order";
    pub const CODE: &'static str = "LELE027";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Delegates,
        summary: "A type's impls in `src/` follow one order: `impl Default`, constructors (receiver-less fns returning `Self`, `impl From`/`TryFrom`/`FromStr`), other trait impls, `pub` fns, then `pub(crate)` fns.",
        why: "Every type file reads the same way: how a value is made, what it is, its API, then its internal machinery.",
        fix: "Reorder the type's impl blocks and items: `impl Default`, constructors and `From`/`TryFrom`/`FromStr`, other trait impls, `pub` fns, then `pub(crate)` fns.",
        bad: rule_examples!("impl_item_order/bad"),
        good: rule_examples!("impl_item_order/good"),
    };
}

#[rustfmt::skip]
impl Rule for ImplItemOrder {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(ImplItemOrder)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl ImplItemOrder {
    pub fn register(rules: &mut Vec<Box<dyn Rule>>) {
        rules.push(Box::new(ImplItemOrder));
    }
}

// no test_usage necessary
