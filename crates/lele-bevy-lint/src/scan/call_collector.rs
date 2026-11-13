use atomic_delegate_macros::atomic_delegates;
use syn::visit::Visit;

pub(crate) struct CallCollector<'a> {
    pub(crate) calls: &'a mut Vec<String>,
    pub(crate) systems: &'a mut Vec<String>,
}

#[atomic_delegates]
impl<'ast> Visit<'ast> for CallCollector<'_> {
    fn visit_expr_call(&mut self, node: &'ast syn::ExprCall) {}
    fn visit_expr_method_call(&mut self, node: &'ast syn::ExprMethodCall) {}
}

#[cfg(test)]
mod tests {
    use syn::visit::Visit;

    use super::CallCollector;

    #[test]
    fn test_usage() {
        let block: syn::Block = syn::parse_quote! {{
            setup();
            app.add_systems(Update, tick);
        }};
        let mut calls = Vec::new();
        let mut systems = Vec::new();
        let mut collector = CallCollector {
            calls: &mut calls,
            systems: &mut systems,
        };
        collector.visit_block(&block);
        assert!(calls.contains(&"setup".to_string()));
        assert!(systems.contains(&"tick".to_string()));
    }
}
