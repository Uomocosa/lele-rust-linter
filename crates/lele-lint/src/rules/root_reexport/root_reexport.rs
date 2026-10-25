use crate::rules;
use crate::Category;
use crate::ExampleFile;
use crate::Project;
use crate::Rule;
use crate::RuleDoc;
use crate::RuleViolation;
use atomic_delegate_macros::atomic_delegate;
use atomic_delegate_macros::atomic_delegates;
use rule_examples_macro::rule_examples;
use std::collections::HashSet;

pub struct RootReexport;

impl RootReexport {
    pub const NAME: &'static str = "root_reexport";
    pub const CODE: &'static str = "LELE015";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Imports,
        summary: "A public type in a crate-root file is re-exported from `lib.rs` (`pub use meters::Meters;`); a root fn file is a private `mod` plus `pub use`.",
        why: "Root items are used as `crate::Meters`, never `crate::meters::Meters`.",
        bad: rule_examples!("root_reexport/bad"),
        good: rule_examples!("root_reexport/good"),
    };
}

#[rustfmt::skip]
impl Rule for RootReexport {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(RootReexport)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl RootReexport {
    pub fn register(rules: &mut Vec<Box<dyn Rule>>) {
        rules.push(Box::new(RootReexport));
    }
    pub(crate) fn public_modules(lib: &syn::File) -> HashSet<String> {
        lib.items
            .iter()
            .filter_map(|item| match item {
                syn::Item::Mod(m) if matches!(m.vis, syn::Visibility::Public(_)) => {
                    Some(m.ident.to_string())
                }
                _ => None,
            })
            .collect()
    }
}

#[atomic_delegates]
impl RootReexport {
    pub(crate) fn missing_fn_flatten(stem: &str, file: &syn::File, lib: &syn::File) -> Vec<String> {
    }
    pub(crate) fn missing_reexports(
        stem: &str,
        file: &syn::File,
        reexported: &HashSet<(String, String)>,
    ) -> Vec<rules::root_reexport::MissingReexport> {
    }
    pub(crate) fn root_reexports(lib: &syn::File) -> HashSet<(String, String)> {}
}

// no test_usage necessary
