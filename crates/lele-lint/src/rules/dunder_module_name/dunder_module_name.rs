use crate::rules;
use crate::traits::Rule;
use crate::Category;
use crate::DunderPath;
use crate::ExampleFile;
use crate::Project;
use crate::RuleDoc;
use crate::RuleViolation;
use atomic_delegate_macros::atomic_delegate;
use atomic_delegate_macros::atomic_delegates;
use rule_examples_macro::rule_examples;
use std::path::Path;

pub struct DunderModuleName;

impl DunderModuleName {
    pub const NAME: &'static str = "dunder_module_name";
    pub const CODE: &'static str = "LELE038";
    pub(crate) const BEVY_OWNED_MOUNT: &'static str = "bevy_systems";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Layout,
        summary: "A dunder folder or file (`__basic__/`) is declared `#[path = \"__basic__/mod.rs\"] mod basic;` under its plain name with the mount visibility the layout gives it — `basic` and the configured methods mount private, `traits`/`bevy_systems` `pub` or `pub(crate)` — and no `pub use`/`pub(crate) use` path goes through a code-dunder mount.",
        why: "The dunders mark the folder on disk (sorted first, not a domain); in code they are noise, so the module carries the plain name. Every item keeps exactly one public path: container items are re-exported flat from the owning module, and trait/system items are reached as `domain::traits::Trait` / `domain::bevy_systems::system`.",
        fix: "Declare the folder as `#[path = \"__basic__/mod.rs\"] mod basic;` (`basic` and the configured methods mount private; `traits`/`bevy_systems` `pub` or `pub(crate)`), write `basic::` in every path, never `__basic__`, and delete every `pub use traits::X;` / `pub use bevy_systems::y;` — refer to the item as `domain::traits::X` / `domain::bevy_systems::y` (`bevy_systems` re-exports are reported by lele-bevy-lint's `bevy_export`).",
        bad: rule_examples!("dunder_module_name/bad"),
        good: rule_examples!("dunder_module_name/good"),
    };
}

#[rustfmt::skip]
impl Rule for DunderModuleName {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(DunderModuleName, Rule)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[atomic_delegates]
impl DunderModuleName {
    pub fn dunder_reexport_hits(
        rel_path: &Path,
        file: &syn::File,
        project: &Project,
    ) -> Vec<rules::dunder_module_name::ReexportHit> {
    }
}

#[rustfmt::skip]
impl DunderModuleName {
    pub fn register(rules: &mut Vec<Box<dyn Rule>>) {
        rules.push(Box::new(DunderModuleName));
    }
}

#[atomic_delegates]
impl DunderModuleName {
    pub(crate) fn dunder_named<'a>(
        dunders: &'a [DunderPath],
        ident: &str,
    ) -> Option<&'a DunderPath> {
    }
    pub(crate) fn dunder_in_path<'a>(
        dunders: &'a [DunderPath],
        target: &str,
    ) -> Option<&'a DunderPath> {
    }
    pub(crate) fn path_attribute(attrs: &[syn::Attribute]) -> Option<String> {}
    pub(crate) fn default_path(dunder: &DunderPath) -> String {}
    pub(crate) fn mount_visibility_hit(
        dunders: &[DunderPath],
        methods_mount_name: &str,
        ident: &str,
        vis: &syn::Visibility,
    ) -> Option<String> {
    }
    pub(crate) fn reexport_suggestion(
        prefix: &[String],
        segments: &[String],
        is_glob: bool,
        mount: &str,
    ) -> String {
    }
    pub(crate) fn mount_prefix(rel_path: &Path, project: &Project) -> Vec<String> {}
    pub(crate) fn collect_reexports(
        visitor: &mut rules::dunder_module_name::DunderNameVisitor,
        item: &syn::ItemUse,
    ) {
    }
    pub(crate) fn render_visibility(vis: &syn::Visibility) -> String {}
}

impl DunderModuleName {
    pub(crate) fn reexport_message(item: &str, suggestion: &str) -> String {
        if item == "*" {
            format!("do not re-export `{suggestion}`")
        } else {
            format!("do not re-export `{item}`: refer to it as `{suggestion}`")
        }
    }
}

// no test_usage necessary
