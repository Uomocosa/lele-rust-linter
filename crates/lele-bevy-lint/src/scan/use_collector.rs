use derive_more::Deref;
use derive_more::DerefMut;
use syn::visit::Visit;

use crate::scan;

#[derive(Deref, DerefMut)]
pub(crate) struct UseCollector<'a>(pub(crate) &'a mut Vec<String>);

#[rustfmt::skip]
impl<'ast> Visit<'ast> for UseCollector<'_> {
    fn visit_expr_call(&mut self, node: &'ast syn::ExprCall) {
        if let syn::Expr::Path(path) = &*node.func && let Some(last) = path.path.segments.last() {
            self.push(last.ident.to_string());
        }
        syn::visit::visit_expr_call(self, node);
    }
    fn visit_expr_method_call(&mut self, node: &'ast syn::ExprMethodCall) {
        if REGISTERING_METHODS.iter().any(|method| node.method == method) {
            let mut names = RegisteredNames(self);
            for arg in &node.args {
                names.visit_expr(arg);
            }
        }
        syn::visit::visit_expr_method_call(self, node);
    }
    fn visit_macro(&mut self, node: &'ast syn::Macro) {
        for expr in scan::MacroCall(node).exprs() {
            self.visit_expr(&expr);
        }
    }
}

#[derive(Deref, DerefMut)]
struct RegisteredNames<'a, 'b>(&'a mut UseCollector<'b>);

#[rustfmt::skip]
impl<'ast> Visit<'ast> for RegisteredNames<'_, '_> {
    fn visit_path(&mut self, node: &'ast syn::Path) {
        if let Some(last) = node.segments.last() {
            self.push(last.ident.to_string());
        }
        syn::visit::visit_path(self, node);
    }
}

const REGISTERING_METHODS: [&str; 7] = [
    "add_systems",
    "add_plugins",
    "add_observer",
    "observe",
    "register_system",
    "run_system_once",
    "run_system_cached",
];

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
