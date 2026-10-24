use derive_more::Deref;
use derive_more::DerefMut;
use syn::visit::Visit;

#[derive(Deref, DerefMut)]
pub(crate) struct ObserverCollector<'a>(pub(crate) &'a mut Vec<String>);

impl<'ast> Visit<'ast> for ObserverCollector<'_> {
    fn visit_path(&mut self, node: &'ast syn::Path) {
        if let Some(last) = node.segments.last() {
            self.push(last.ident.to_string());
        }
        syn::visit::visit_path(self, node);
    }
}

#[cfg(test)]
mod tests {
    use syn::visit::Visit;

    use super::ObserverCollector;

    #[test]
    fn test_usage() {
        let expr: syn::Expr = syn::parse_str("tick").unwrap();
        let mut found = Vec::new();
        let mut collector = ObserverCollector(&mut found);
        collector.visit_expr(&expr);
        assert_eq!(found, vec!["tick".to_string()]);
    }
}
