use atomic_delegate_macros::atomic_delegates;
use derive_more::Deref;
use derive_more::DerefMut;
use syn::visit::Visit;

#[derive(Deref, DerefMut)]
pub(crate) struct SelfCalls<'a>(pub(crate) &'a mut Vec<String>);

#[atomic_delegates]
impl<'ast> Visit<'ast> for SelfCalls<'_> {
    fn visit_expr_method_call(&mut self, node: &'ast syn::ExprMethodCall) {}
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
