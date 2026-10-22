use std::collections::BTreeSet;

use atomic_delegate_macros::atomic_delegates;
use derive_more::Deref;
use lele_lint::Project;
use lele_lint::RuleViolation;

use crate::scan;

#[derive(Clone, Copy, Deref)]
pub struct ScannedFile<'a>(pub &'a syn::File);

#[atomic_delegates]
impl ScannedFile<'_> {
    pub fn prod_visuals(&self, extra: &BTreeSet<String>) -> Vec<scan::FoundVisual> {}
    pub fn restyled_systems(&self, graph: &scan::CallGraph) -> Vec<scan::Restyle> {}
    pub fn declared_components(&self) -> Vec<String> {}
    pub fn time_drivers(&self) -> Vec<String> {}
    pub fn reachable_visual(
        &self,
        graph: &scan::CallGraph,
        extra: &BTreeSet<String>,
    ) -> Option<scan::FoundVisual> {
    }
    pub fn preview_subjects(
        &self,
        graph: &scan::CallGraph,
        extra: &BTreeSet<String>,
    ) -> Vec<String> {
    }
    pub fn input_drivers(&self) -> Vec<String> {}
    pub fn collect_previews(&self) -> Vec<scan::Preview> {}
    pub fn imports_harness_run(&self) -> bool {}
    pub fn require_preview(
        &self,
        kind: scan::PreviewKind,
        missing: impl FnOnce() -> RuleViolation,
    ) -> Option<RuleViolation> {
    }
    pub fn module_fns(&self) -> scan::ModuleFns<'_> {}
    pub fn plugin_registrations(&self, project: &Project) -> Vec<Vec<String>> {}
}

#[rustfmt::skip]
impl ScannedFile<'_> {
    pub fn defines_plugin(&self) -> bool {
        self.items.iter().any(|item| match item {
            syn::Item::Impl(impl_block) => impl_block.trait_.as_ref().is_some_and(|(_, path, _)| path.segments.last().is_some_and(|segment| segment.ident == "Plugin")),
            _ => false,
        })
    }
    pub fn file_reaches_production(&self, graph: &scan::CallGraph) -> bool {
        self.items.iter().any(|item| match item {
            syn::Item::Fn(func) => graph.reaches_production(&func.sig.ident.to_string()),
            syn::Item::Impl(impl_block) => impl_block.items.iter().any(|item| matches!(item, syn::ImplItem::Fn(method) if graph.reaches_production(&method.sig.ident.to_string()))),
            _ => false,
        })
    }
    pub fn plugin_types(&self) -> Vec<String> {
        self.items.iter().filter_map(|item| match item { syn::Item::Impl(impl_block) => scan::PluginImpl(impl_block).plugin_type(), _ => None }).collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::scan::ScannedFile;

    #[test]
    fn test_usage() {
        let file: syn::File = syn::parse_quote! {
            pub struct GamePlugin;
            impl Plugin for GamePlugin { fn build(&self, app: &mut App) {} }
        };
        let scanned = ScannedFile(&file);
        assert!(scanned.defines_plugin());
        assert_eq!(scanned.plugin_types(), vec!["GamePlugin".to_string()]);
        assert!(scanned.collect_previews().is_empty());
    }
}
