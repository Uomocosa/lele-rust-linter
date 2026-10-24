use derive_more::Deref;
use derive_more::DerefMut;
use syn::visit::Visit;

use crate::scan;

#[derive(Deref, DerefMut)]
pub(crate) struct LocalNames<'a>(pub(crate) &'a mut Vec<String>);

#[rustfmt::skip]
impl<'ast> Visit<'ast> for LocalNames<'_> {
    fn visit_path(&mut self, node: &'ast syn::Path) {
        if let [only] = node.segments.iter().collect::<Vec<_>>().as_slice() {
            self.push(only.ident.to_string());
        }
        syn::visit::visit_path(self, node);
    }
    fn visit_macro(&mut self, node: &'ast syn::Macro) {
        for expr in scan::MacroCall(node).exprs() {
            self.visit_expr(&expr);
        }
    }
}

#[cfg(test)]
mod tests {
    use syn::visit::Visit;

    use super::LocalNames;

    #[test]
    fn test_usage() {
        let expr: syn::Expr = syn::parse_str("run(&scene(), &config)").unwrap();
        let mut found = Vec::new();
        let mut collector = LocalNames(&mut found);
        collector.visit_expr(&expr);
        assert!(found.contains(&"run".to_string()));
        assert!(found.contains(&"scene".to_string()));
        assert!(found.contains(&"config".to_string()));
    }
}
