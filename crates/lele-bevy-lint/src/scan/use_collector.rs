use crate::scan;
use atomic_delegate_macros::atomic_delegate;
use derive_more::Deref;
use derive_more::DerefMut;
use syn::visit::Visit;

#[derive(Deref, DerefMut)]
pub(crate) struct UseCollector<'a>(pub(crate) &'a mut Vec<String>);

impl<'ast> Visit<'ast> for UseCollector<'_> {
    #[atomic_delegate(UseCollector, Visit)]
    fn visit_expr_call(&mut self, node: &'ast syn::ExprCall) {}
    #[atomic_delegate(UseCollector, Visit)]
    fn visit_expr_method_call(&mut self, node: &'ast syn::ExprMethodCall) {}
    fn visit_macro(&mut self, node: &'ast syn::Macro) {
        for expr in scan::MacroCall(node).exprs() {
            self.visit_expr(&expr);
        }
    }
}

#[cfg(test)]
mod tests {
    use syn::visit::Visit;

    use super::UseCollector;

    #[test]
    fn test_usage() {
        let expr: syn::Expr =
            syn::parse_str("app.add_systems(Startup, super::spawn_root)").unwrap();
        let mut found = Vec::new();
        let mut collector = UseCollector(&mut found);
        collector.visit_expr(&expr);
        assert!(found.contains(&"spawn_root".to_string()));
    }
}
