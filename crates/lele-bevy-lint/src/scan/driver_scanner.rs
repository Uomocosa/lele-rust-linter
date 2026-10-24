use derive_more::Deref;
use derive_more::DerefMut;
use syn::visit::Visit;

use crate::TIME_DRIVERS;

#[derive(Deref, DerefMut)]
pub(crate) struct DriverScanner<'a>(pub(crate) &'a mut Vec<String>);

#[rustfmt::skip]
impl<'ast> Visit<'ast> for DriverScanner<'_> {
    fn visit_path(&mut self, node: &'ast syn::Path) {
        for segment in &node.segments {
            let name = segment.ident.to_string();
            if TIME_DRIVERS.contains(&name.as_str()) {
                self.push(name);
            }
        }
        syn::visit::visit_path(self, node);
    }
    fn visit_expr_method_call(&mut self, node: &'ast syn::ExprMethodCall) {
        let name = node.method.to_string();
        if TIME_DRIVERS.contains(&name.as_str()) {
            self.push(name);
        }
        syn::visit::visit_expr_method_call(self, node);
    }
}

#[cfg(test)]
mod tests {
    use syn::visit::Visit;

    use super::DriverScanner;

    #[test]
    fn test_usage() {
        let item: syn::Item =
            syn::parse_str("pub fn tick(time: Res<Time>) { let _ = time.delta_secs(); }").unwrap();
        let mut found = Vec::new();
        let mut scanner = DriverScanner(&mut found);
        scanner.visit_item(&item);
        assert!(found.contains(&"Time".to_string()));
        assert!(found.contains(&"delta_secs".to_string()));
    }
}
