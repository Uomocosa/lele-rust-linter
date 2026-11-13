use std::path::Path;

use crate::traits::Rule;
use crate::Category;
use crate::ExampleFile;
use crate::Project;
use crate::RuleDoc;
use crate::RuleViolation;
use atomic_delegate_macros::atomic_delegate;
use atomic_delegate_macros::atomic_delegates;
use rule_examples_macro::rule_examples;

pub struct InlineTraitMethods;

impl InlineTraitMethods {
    pub const NAME: &'static str = "inline_trait_methods";
    pub const CODE: &'static str = "LELE043";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Delegates,
        summary: "A trait method listed in `[[lele.lint.inline_trait_methods]]` (trait path plus method names, with a reason) keeps its body inline in the trait impl: it is not delegated to `__methods__/`, it has no `#[atomic_delegate]` or `#[atomic_delegates]` annotation, its body is not empty, and it is not a single call to a delegate shell of the same type. The rule also checks each listed entry: it needs a reason, at least one method, and no method named twice.",
        why: "Some traits are called by a framework that expects the method in the impl itself. Bevy's `Plugin::build` is the usual case: the trait method is the registration point, and a body that only forwards to an inherent shell hides the wiring the framework reads. The reason is recorded in the config so the exception is a decision, not an accident.",
        fix: "Write the listed method's body inline in the trait impl and delete its `__methods__/<type>/<file>.rs` body. Remove any `#[atomic_delegate]` on the method and any `#[atomic_delegates]` on the impl block. Only the user adds or changes `[[lele.lint.inline_trait_methods]]` entries; a trait path matches an impl when the last segments agree (`bevy::app::Plugin` matches `impl Plugin` and `impl bevy::prelude::Plugin`, and `impl other::Plugin` does not).",
        bad: rule_examples!("inline_trait_methods/bad"),
        good: rule_examples!("inline_trait_methods/good"),
    };
}

#[rustfmt::skip]
impl Rule for InlineTraitMethods {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(InlineTraitMethods, Rule)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl InlineTraitMethods {
    pub fn register(rules: &mut Vec<Box<dyn Rule>>) {
        rules.push(Box::new(InlineTraitMethods));
    }
}

#[atomic_delegates]
impl InlineTraitMethods {
    pub(crate) fn listed_methods(project: &Project, impl_block: &syn::ItemImpl) -> Vec<String> {}
    pub(crate) fn without_listed_methods(
        project: &Project,
        impl_block: &syn::ItemImpl,
    ) -> syn::ItemImpl {
    }
    pub(crate) fn trait_path_matches(trait_path: &str, impl_trait: &syn::Path) -> bool {}
    pub(crate) fn impl_block_violations(
        project: &Project,
        rel_path: &Path,
        file: &syn::File,
        impl_block: &syn::ItemImpl,
    ) -> Vec<RuleViolation> {
    }
    pub(crate) fn called_shell_file(
        project: &Project,
        file: &syn::File,
        impl_block: &syn::ItemImpl,
        method: &syn::ImplItemFn,
    ) -> Option<String> {
    }
    pub(crate) fn config_violations(project: &Project) -> Vec<RuleViolation> {}
}

// no test_usage necessary
