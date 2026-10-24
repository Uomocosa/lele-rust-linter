use derive_more::Deref;
use derive_more::DerefMut;
use syn::visit::Visit;

use lele_lint::ItemMarker;

use crate::scan;

#[derive(Deref, DerefMut)]
pub(crate) struct RegisteredCollector<'a>(pub(crate) &'a mut Vec<String>);

#[rustfmt::skip]
impl<'ast> Visit<'ast> for RegisteredCollector<'_> {
    fn visit_item_mod(&mut self, node: &'ast syn::ItemMod) {
        if node.attrs.iter().any(|attr| ItemMarker(attr).is_cfg_test()) {
            return;
        }
        syn::visit::visit_item_mod(self, node);
    }
    fn visit_expr_method_call(&mut self, node: &'ast syn::ExprMethodCall) {
        if let Some(idents) = scan::AppMethodCall(node).system_args() {
            self.extend(idents);
            self.visit_expr(&node.receiver);
        } else {
            syn::visit::visit_expr_method_call(self, node);
        }
    }
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
