use atomic_delegate_macros::atomic_delegates;
use derive_more::Deref;
use derive_more::DerefMut;
use syn::visit::Visit;

#[derive(Deref, DerefMut)]
pub(crate) struct DriverScanner<'a>(pub(crate) &'a mut Vec<String>);

#[atomic_delegates]
impl<'ast> Visit<'ast> for DriverScanner<'_> {
    fn visit_path(&mut self, node: &'ast syn::Path) {}
    fn visit_expr_method_call(&mut self, node: &'ast syn::ExprMethodCall) {}
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
