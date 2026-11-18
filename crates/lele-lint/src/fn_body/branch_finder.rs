use derive_more::Deref;
use derive_more::DerefMut;
use syn::visit::Visit;

#[derive(Default, Deref, DerefMut)]
pub(crate) struct BranchFinder(pub(crate) bool);

impl<'ast> Visit<'ast> for BranchFinder {
    fn visit_expr(&mut self, node: &'ast syn::Expr) {
        if matches!(
            node,
            syn::Expr::If(_)
                | syn::Expr::Match(_)
                | syn::Expr::ForLoop(_)
                | syn::Expr::While(_)
                | syn::Expr::Loop(_)
                | syn::Expr::Try(_)
                | syn::Expr::Closure(_)
                | syn::Expr::Await(_)
        ) {
            **self = true;
        }
        syn::visit::visit_expr(self, node);
    }
}

#[cfg(test)]
mod tests {
    use super::BranchFinder;
    use syn::visit::Visit;

    #[test]
    fn test_usage() {
        let mut finder = BranchFinder::default();
        let plain: syn::Block = syn::parse_quote! {{ Self { port: 8080 } }};
        finder.visit_block(&plain);
        assert!(!*finder);
        let question: syn::Block = syn::parse_quote! {{ fetch(port)? }};
        finder.visit_block(&question);
        assert!(*finder);
    }

    #[test]
    fn test_usage_closure_and_await_are_branching() {
        let mut finder = BranchFinder::default();
        let closure: syn::Block = syn::parse_quote! {{ items.iter().map(|x| x + 1).sum() }};
        finder.visit_block(&closure);
        assert!(*finder);
        let mut awaited = BranchFinder::default();
        let pending: syn::Block = syn::parse_quote! {{ handle.await }};
        awaited.visit_block(&pending);
        assert!(*awaited);
    }
}
