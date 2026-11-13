use crate::traits::Rule;
use crate::Category;
use crate::ExampleFile;
use crate::Project;
use crate::RuleDoc;
use crate::RuleViolation;
use atomic_delegate_macros::atomic_delegate;
use atomic_delegate_macros::atomic_delegates;
use rule_examples_macro::rule_examples;
use std::collections::HashMap;
use std::collections::HashSet;

pub struct NoTrivialAccessors;

impl NoTrivialAccessors {
    pub const NAME: &'static str = "no_trivial_accessors";
    pub const CODE: &'static str = "LELE014";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Types,
        summary: "No getters or setters that only read or write a public field, including through `Ok(..)`/`Some(..)` or a plain conversion (`.clone()`, `.as_str()`, ...).",
        why: "The field is already public; an accessor adds a second name for the same thing.",
        fix: "Delete the getter or setter and make callers read or write the public field directly.",
        bad: rule_examples!("no_trivial_accessors/bad"),
        good: rule_examples!("no_trivial_accessors/good"),
    };
}

#[rustfmt::skip]
impl Rule for NoTrivialAccessors {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(NoTrivialAccessors, Rule)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl NoTrivialAccessors {
    pub fn register(rules: &mut Vec<Box<dyn Rule>>) {
        rules.push(Box::new(NoTrivialAccessors));
    }
}

#[atomic_delegates]
impl NoTrivialAccessors {
    pub(crate) fn vis_to_string(vis: &syn::Visibility) -> Option<String> {}
    pub(crate) fn filtered_stmts(block: &syn::Block) -> Vec<&syn::Stmt> {}
    pub(crate) fn is_ok_or_some(call: &syn::ExprCall) -> bool {}
    pub(crate) fn collect_vis_fields(file: &syn::File) -> HashMap<String, String> {}
    pub(crate) fn is_builder(method: &syn::ImplItemFn) -> bool {}
    pub(crate) fn is_log_macro(stmt: &syn::Stmt) -> bool {}
    pub(crate) fn is_trivial_accessor(
        method: &syn::ImplItemFn,
        pub_fields: &HashSet<String>,
    ) -> Option<String> {
    }
    pub(crate) fn is_trivial_setter(
        method: &syn::ImplItemFn,
        pub_fields: &HashSet<String>,
    ) -> Option<String> {
    }
    pub(crate) fn extract_assign_field(
        assign: &syn::ExprAssign,
        sig: &syn::Signature,
        pub_fields: &HashSet<String>,
    ) -> Option<String> {
    }
    pub(crate) fn extract_self_field(
        expr: &syn::Expr,
        pub_fields: &HashSet<String>,
    ) -> Option<String> {
    }
    pub(crate) fn is_plain_conversion(call: &syn::ExprMethodCall) -> bool {}
    pub(crate) fn is_self_ref(expr: &syn::Expr) -> bool {}
}

// no test_usage necessary
