use derive_more::Deref;
use derive_more::DerefMut;
use syn::visit::Visit;

#[derive(Deref, DerefMut)]
pub(crate) struct SelfCalls<'a>(pub(crate) &'a mut Vec<String>);

#[rustfmt::skip]
impl<'ast> Visit<'ast> for SelfCalls<'_> {
    fn visit_expr_method_call(&mut self, node: &'ast syn::ExprMethodCall) {
        if let syn::Expr::Path(receiver) = &*node.receiver
            && receiver.path.is_ident("self")
        {
            self.push(node.method.to_string());
        }
        syn::visit::visit_expr_method_call(self, node);
    }
}

#[cfg(test)]
mod tests {
    use syn::visit::Visit;

    use super::SelfCalls;

    #[test]
    fn test_usage() {
        let block: syn::Block = syn::parse_quote! {{ self.build_plugin(app); }};
        let mut names = Vec::new();
        let mut collector = SelfCalls(&mut names);
        collector.visit_block(&block);
        assert_eq!(names, vec!["build_plugin".to_string()]);
    }
}
