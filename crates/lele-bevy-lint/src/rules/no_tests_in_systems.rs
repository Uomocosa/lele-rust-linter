use std::collections::HashSet;

use atomic_delegate_macros::atomic_delegate;
use atomic_delegate_macros::atomic_delegates;
use lele_lint::Category;
use lele_lint::ExampleFile;
use lele_lint::ItemMarker;
use lele_lint::Project;
use lele_lint::RuleDoc;
use lele_lint::RuleViolation;
use lele_lint::traits::Rule;
use rule_examples_macro::rule_examples;
use syn::spanned::Spanned;

pub struct NoTestsInSystems;

impl NoTestsInSystems {
    pub const NAME: &'static str = "no_tests_in_systems";
    pub const CODE: &'static str = "LELEBEVYTEST001";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Tests,
        summary: "No `#[cfg(test)]` in a file under a `__bevy_systems__/` (or `bevy_systems/`) folder, except a test module holding only UI preview tests (plus `use`/`const`).",
        why: "A Bevy system is glue between queries and the honest core; the core is tested where it lives and the wiring once at plugin level.",
        fix: "Delete the test module. If a test checks behaviour a user of the plugin can see and no other test covers it, move it to the owning `*_plugin.rs` tests, building the app with the plugin. Previews are the visual usage of a UI system and stay colocated: a `#[cfg(test)]` module holding only `*_ui_*_preview` tests (plus `use`/`const`) is allowed.",
        bad: rule_examples!("no_tests_in_systems/bad"),
        good: rule_examples!("no_tests_in_systems/good"),
    };
}

#[rustfmt::skip]
impl Rule for NoTestsInSystems {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(NoTestsInSystems, Rule)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl NoTestsInSystems {
    pub fn register(rules: &mut Vec<Box<dyn Rule>>) {
        rules.push(Box::new(Self));
    }
}

impl NoTestsInSystems {
    pub(crate) fn cfg_test_line(item: &syn::Item) -> Option<usize> {
        let attrs: &[syn::Attribute] = match item {
            syn::Item::Const(found) => &found.attrs,
            syn::Item::Enum(found) => &found.attrs,
            syn::Item::ExternCrate(found) => &found.attrs,
            syn::Item::Fn(found) => &found.attrs,
            syn::Item::ForeignMod(found) => &found.attrs,
            syn::Item::Impl(found) => &found.attrs,
            syn::Item::Macro(found) => &found.attrs,
            syn::Item::Mod(found) => &found.attrs,
            syn::Item::Static(found) => &found.attrs,
            syn::Item::Struct(found) => &found.attrs,
            syn::Item::Trait(found) => &found.attrs,
            syn::Item::TraitAlias(found) => &found.attrs,
            syn::Item::Type(found) => &found.attrs,
            syn::Item::Union(found) => &found.attrs,
            syn::Item::Use(found) => &found.attrs,
            syn::Item::Verbatim(_) | _ => return None,
        };
        attrs
            .iter()
            .any(|attr| ItemMarker(attr).is_cfg_test())
            .then(|| item.span().start().line)
    }

    pub(crate) fn is_allowed_preview_item(item: &syn::Item, previews: &HashSet<String>) -> bool {
        match item {
            syn::Item::Use(_) | syn::Item::Const(_) => true,
            syn::Item::Fn(func) => previews.contains(&func.sig.ident.to_string()),
            _ => false,
        }
    }
}

#[atomic_delegates]
impl NoTestsInSystems {
    pub(crate) fn cfg_test_module(item: &syn::Item) -> Option<&syn::ItemMod> {}
    pub(crate) fn preview_names(items: &[syn::Item]) -> HashSet<String> {}
}

// no test_usage necessary
