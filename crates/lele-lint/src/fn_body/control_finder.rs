use derive_more::Deref;
use derive_more::DerefMut;
use syn::visit::Visit;

#[derive(Default, Deref, DerefMut)]
pub(crate) struct ControlFinder(pub(crate) bool);

impl<'ast> Visit<'ast> for ControlFinder {
    fn visit_expr(&mut self, node: &'ast syn::Expr) {
        if matches!(
            node,
            syn::Expr::Match(_)
                | syn::Expr::If(_)
                | syn::Expr::ForLoop(_)
                | syn::Expr::While(_)
                | syn::Expr::Loop(_)
        ) {
            **self = true;
        }
        syn::visit::visit_expr(self, node);
    }
}

#[cfg(test)]
mod tests {
    use super::ControlFinder;
    use syn::visit::Visit;

    #[test]
    fn test_usage() {
        let mut finder = ControlFinder::default();
        let plain: syn::Block = syn::parse_quote! {{ g(1) }};
        finder.visit_block(&plain);
        assert!(!*finder);
        let nested: syn::Block = syn::parse_quote! {{ h(|x| if x { 1 } else { 0 }) }};
        finder.visit_block(&nested);
        assert!(*finder);
    }
}
