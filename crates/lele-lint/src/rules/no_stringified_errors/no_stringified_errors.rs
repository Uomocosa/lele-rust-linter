use crate::traits::Rule;
use crate::Category;
use crate::ExampleFile;
use crate::Project;
use crate::RuleDoc;
use crate::RuleViolation;
use atomic_delegate_macros::atomic_delegate;
use atomic_delegate_macros::atomic_delegates;
use rule_examples_macro::rule_examples;
use std::collections::HashSet;

pub struct NoStringifiedErrors;

impl NoStringifiedErrors {
    pub const NAME: &'static str = "no_stringified_errors";
    pub const CODE: &'static str = "LELE039";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Types,
        summary: "A caught error is never stored as text in the crate's own error enum (`map_err`, `impl From`, or a helper taking `…Error`): keep it as a `#[source]`/`#[from]` field.",
        why: "Text throws away the error's type, its `source()` chain and `?` conversions; callers can only print it, never match or inspect it.",
        fix: "Keep the caught error as a typed field: `#[from] source: io::Error` or `#[source]`, instead of `.to_string()` or `format!`.",
        bad: rule_examples!("no_stringified_errors/bad"),
        good: rule_examples!("no_stringified_errors/good"),
    };
}

#[rustfmt::skip]
impl Rule for NoStringifiedErrors {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(NoStringifiedErrors)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl NoStringifiedErrors {
    pub fn register(rules: &mut Vec<Box<dyn Rule>>) {
        rules.push(Box::new(NoStringifiedErrors));
    }
}

impl NoStringifiedErrors {
    pub(crate) fn names_an_error_type(ty: &syn::Type) -> bool {
        match ty {
            syn::Type::Reference(reference) => Self::names_an_error_type(&reference.elem),
            syn::Type::Path(path) => path
                .path
                .segments
                .last()
                .is_some_and(|segment| segment.ident.to_string().ends_with("Error")),
            _ => false,
        }
    }
    pub(crate) fn last_ident(ty: &syn::Type) -> Option<String> {
        match ty {
            syn::Type::Path(path) => path
                .path
                .segments
                .last()
                .map(|segment| segment.ident.to_string()),
            _ => None,
        }
    }
}

#[atomic_delegates]
impl NoStringifiedErrors {
    pub(crate) fn own_error_enums(project: &Project) -> HashSet<String> {}
    pub(crate) fn mentions_name(tokens: proc_macro2::TokenStream, name: &str) -> bool {}
}

// no test_usage necessary
