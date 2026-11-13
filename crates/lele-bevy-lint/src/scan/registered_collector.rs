use atomic_delegate_macros::atomic_delegates;
use derive_more::Deref;
use derive_more::DerefMut;
use syn::visit::Visit;

#[derive(Deref, DerefMut)]
pub(crate) struct RegisteredCollector<'a>(pub(crate) &'a mut Vec<String>);

#[atomic_delegates]
impl<'ast> Visit<'ast> for RegisteredCollector<'_> {
    fn visit_item_mod(&mut self, node: &'ast syn::ItemMod) {}
    fn visit_expr_method_call(&mut self, node: &'ast syn::ExprMethodCall) {}
}

#[cfg(test)]
mod tests {
    use syn::visit::Visit;

    use super::RegisteredCollector;

    #[test]
    fn test_usage() {
        let file: syn::File =
            syn::parse_str("pub fn build(app: &mut App) { app.add_systems(Update, tick); }")
                .unwrap();
        let mut found = Vec::new();
        let mut collector = RegisteredCollector(&mut found);
        collector.visit_file(&file);
        assert!(found.contains(&"tick".to_string()));
    }
}
