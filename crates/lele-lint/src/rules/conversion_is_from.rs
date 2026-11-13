use crate::traits::Rule;
use crate::Category;
use crate::ExampleFile;
use crate::Project;
use crate::RuleDoc;
use crate::RuleViolation;
use atomic_delegate_macros::atomic_delegate;
use atomic_delegate_macros::atomic_delegates;
use rule_examples_macro::rule_examples;

pub struct ConversionIsFrom;

impl ConversionIsFrom {
    pub const NAME: &'static str = "conversion_is_from";
    pub const CODE: &'static str = "LELE036";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Types,
        summary: "An associated fn that builds `Self` (or `Result<Self, _>`) from exactly one argument is a conversion: write it as `impl From<A> for Self` (`TryFrom<A>` when fallible). `new`, `Option<Self>`, trait impls and fns taking a `Self` argument (merges) are exempt. `async fn`, `const fn` and fns whose argument is generic (`impl Trait` or a type parameter) cannot be written as a trait impl and are exempt. Types with two or more one-argument builders sharing one argument type are named constructors (for example `Duration::from_secs` / `from_millis`), not conversions, and are exempt.",
        why: "A conversion has one standard name in Rust; `Reading::from(x)` and `x.into()` work everywhere, while `Reading::from_sensor(x)` has to be looked up. A named argument type says what the number or string means at the call site (this avoids primitive obsession).",
        fix: "Replace the one-argument builder with `impl From<A> for <Type>` (`impl TryFrom<A>` if it returns `Result`) and update callers to `Type::from(a)` or `a.into()`. When the argument is a bare primitive (`usize`, `&str`, `String`, ...), wrap it in a named newtype first and convert from that: `Deck::from(deck::Capacity(52))`, not `Deck::from(52)`.",
        bad: rule_examples!("conversion_is_from/bad"),
        good: rule_examples!("conversion_is_from/good"),
    };
}

#[rustfmt::skip]
impl Rule for ConversionIsFrom {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(ConversionIsFrom)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl ConversionIsFrom {
    pub fn register(rules: &mut Vec<Box<dyn Rule>>) {
        rules.push(Box::new(ConversionIsFrom));
    }
    pub(crate) fn is_exempt_from_conversion(sig: &syn::Signature) -> bool { sig.asyncness.is_some() || sig.constness.is_some() || Self::has_generic_argument(sig) }
}

#[atomic_delegates]
impl ConversionIsFrom {
    pub(crate) fn takes_self_typed_argument(sig: &syn::Signature, self_name: &str) -> bool {}
    pub(crate) fn is_self(ty: &syn::Type, self_name: &str) -> bool {}
    pub(crate) fn advice(sig: &syn::Signature, self_name: &str, shape: &str) -> Option<String> {}
    pub(crate) fn has_generic_argument(sig: &syn::Signature) -> bool {}
    pub(crate) fn constructed_shape(
        output: &syn::ReturnType,
        self_name: &str,
    ) -> Option<&'static str> {
    }
    pub(crate) fn single_argument_type(sig: &syn::Signature) -> Option<String> {}
}

// no test_usage necessary
