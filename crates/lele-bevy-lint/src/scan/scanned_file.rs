use std::collections::BTreeSet;
use std::collections::HashSet;

use atomic_delegate_macros::atomic_delegates;
use derive_more::Deref;
use lele_lint::Project;
use lele_lint::RuleViolation;
use syn::visit::Visit;

use crate::STYLING_COMPONENTS;
use crate::scan;
use crate::scan::basic::constants::HARNESS_CRATE;
use crate::scan::basic::constants::HARNESS_RUN;

#[derive(Clone, Copy, Deref)]
pub struct ScannedFile<'a>(pub &'a syn::File);

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
    pub(crate) fn styling_aliases(file: &syn::File) -> HashSet<String> {}
    pub(crate) fn self_calls(block: &syn::Block) -> Vec<String> {}
    pub(crate) fn called_sibling<'b>(
        arg: &syn::Expr,
        module_fns: &scan::ModuleFns<'b>,
    ) -> Option<&'b syn::ItemFn> {
    }
}

#[rustfmt::skip]
impl ScannedFile<'_> {
    pub(crate) fn derives_component(attrs: &[syn::Attribute]) -> bool {
        attrs.iter().filter(|attr| attr.path().is_ident("derive")).any(|attr| {
            attr.parse_args_with(syn::punctuated::Punctuated::<syn::Path, syn::Token![,]>::parse_terminated).is_ok_and(|paths| {
                paths.iter().any(|path| path.segments.last().is_some_and(|segment| segment.ident == "Component"))
            })
        })
    }
    pub(crate) fn signature_restyles(sig: &syn::Signature, aliases: &HashSet<String>) -> bool {
        sig.inputs.iter().any(|input| { let syn::FnArg::Typed(pat) = input else { return false; }; let mut detector = scan::StylingDetector { aliases, found: false, }; detector.visit_type(&pat.ty); detector.found })
    }
    pub(crate) fn is_styling(ty: &syn::Type) -> bool {
        let syn::Type::Path(path) = ty else { return false; };
        path.path.segments.last().is_some_and(|segment| STYLING_COMPONENTS.contains(&segment.ident.to_string().as_str()))
    }
    pub(crate) fn imports_run(tree: &syn::UseTree, is_under_harness: bool) -> bool {
        match tree { syn::UseTree::Path(path) => Self::imports_run(&path.tree, is_under_harness || path.ident == HARNESS_CRATE), syn::UseTree::Name(name) => is_under_harness && name.ident == HARNESS_RUN, syn::UseTree::Rename(rename) => is_under_harness && rename.ident == HARNESS_RUN, syn::UseTree::Glob(_) => is_under_harness, syn::UseTree::Group(group) => group.items.iter().any(|item| Self::imports_run(item, is_under_harness)), }
    }
    pub(crate) fn kind_of(name: &str) -> Option<scan::PreviewKind> {
        if name.ends_with(PNG_SUFFIX) { Some(scan::PreviewKind::Png) } else if name.ends_with(MP4_SUFFIX) { Some(scan::PreviewKind::Mp4) } else if name.ends_with(SCENE_SUFFIX) { Some(scan::PreviewKind::Scene) } else { None }
    }
    pub(crate) fn module_fns_of(items: &[syn::Item]) -> scan::ModuleFns<'_> {
        items.iter().filter_map(|item| match item { syn::Item::Fn(func) => Some((func.sig.ident.to_string(), func)), _ => None, }).collect()
    }
    pub(crate) fn block_expr(block: &syn::Block) -> syn::Expr {
        let inner = syn::ExprBlock { attrs: Vec::new(), label: None, block: block.clone(), };
        syn::Expr::Block(inner)
    }
}

const PNG_SUFFIX: &str = "_ui_png_preview";
const MP4_SUFFIX: &str = "_ui_mp4_preview";
const SCENE_SUFFIX: &str = "_ui_scene_preview";

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
