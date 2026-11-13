use atomic_delegate_macros::atomic_delegates;
use derive_more::Deref;
use derive_more::DerefMut;
use syn::visit::Visit;

#[derive(Deref, DerefMut)]
pub(crate) struct InputScanner<'a>(pub(crate) &'a mut Vec<String>);

#[atomic_delegates]
impl<'ast> Visit<'ast> for InputScanner<'_> {
    fn visit_path(&mut self, node: &'ast syn::Path) {}
    fn visit_expr_method_call(&mut self, node: &'ast syn::ExprMethodCall) {}
}

#[cfg(test)]
mod tests {
    use syn::visit::Visit;

    use super::InputScanner;

    #[test]
    fn test_usage() {
        let item: syn::Item =
            syn::parse_str("pub fn gate(keys: Res<ButtonInput<KeyCode>>) { let _ = keys; }")
                .unwrap();
        let mut found = Vec::new();
        let mut scanner = InputScanner(&mut found);
        scanner.visit_item(&item);
        assert!(found.contains(&"ButtonInput".to_string()));
    }
}
