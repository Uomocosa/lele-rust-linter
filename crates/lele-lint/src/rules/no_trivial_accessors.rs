use crate::Category;
use crate::ExampleFile;
use crate::Project;
use crate::Rule;
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
        bad: rule_examples!("no_trivial_accessors/bad"),
        good: rule_examples!("no_trivial_accessors/good"),
    };
}

#[rustfmt::skip]
impl Rule for NoTrivialAccessors {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(NoTrivialAccessors)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl NoTrivialAccessors {
    pub fn register(rules: &mut Vec<Box<dyn Rule>>) {
        rules.push(Box::new(NoTrivialAccessors));
    }
    pub(crate) fn vis_to_string(vis: &syn::Visibility) -> Option<String> {
        match vis {
            syn::Visibility::Public(_) => Some("pub".to_string()),
            syn::Visibility::Restricted(res) => {
                let s = quote::quote!(#res).to_string();
                Some(s.replace(" )", ")").replace("( ", "("))
            }
            syn::Visibility::Inherited => None,
        }
    }
    pub(crate) fn filtered_stmts(block: &syn::Block) -> Vec<&syn::Stmt> {
        block.stmts.iter().filter(|s| !Self::is_log_macro(s)).collect()
    }
    pub(crate) fn is_ok_or_some(call: &syn::ExprCall) -> bool {
        matches!(&*call.func, syn::Expr::Path(path)
            if path.path.segments.last().is_some_and(|segment| segment.ident == "Ok" || segment.ident == "Some"))
    }
}

#[atomic_delegates]
impl NoTrivialAccessors {
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
