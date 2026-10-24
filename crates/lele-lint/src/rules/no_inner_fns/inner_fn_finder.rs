use atomic_delegate_macros::atomic_delegates;
use syn::visit::Visit;

#[derive(Default)]
pub(crate) struct InnerFnFinder<'ast> {
    pub(crate) fn_depth: usize,
    pub(crate) inner_fns: Vec<&'ast syn::ItemFn>,
}

#[atomic_delegates]
impl<'ast> Visit<'ast> for InnerFnFinder<'ast> {
    fn visit_item_fn(&mut self, item: &'ast syn::ItemFn) {}
    fn visit_impl_item_fn(&mut self, item: &'ast syn::ImplItemFn) {}
    fn visit_trait_item_fn(&mut self, item: &'ast syn::TraitItemFn) {}
    fn visit_item_mod(&mut self, item: &'ast syn::ItemMod) {}
}

#[cfg(test)]
mod tests {
    use syn::visit::Visit;

    use super::InnerFnFinder;

    #[test]
    fn test_usage() {
        let file: syn::File = syn::parse_str(
            "fn outer() { fn helper() {} fn recursion() {} } impl A { fn m() { fn nested() {} } }",
        )
        .unwrap();
        let mut finder = InnerFnFinder::default();
        finder.visit_file(&file);
        let names: Vec<String> = finder
            .inner_fns
            .iter()
            .map(|f| f.sig.ident.to_string())
            .collect();
        assert_eq!(names, ["helper", "nested"]);
    }
}
