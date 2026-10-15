use derive_more::{Deref, DerefMut};
use syn::visit::Visit;

use crate::scan;

#[must_use]
pub fn scene_literals(region: &[syn::Expr]) -> Vec<syn::ExprStruct> {
    let mut found = Vec::new();
    let mut finder = SceneFinder(&mut found);
    for expr in region {
        finder.visit_expr(expr);
    }
    found
}

// needed helper: collects `Scene { build, .. }` literals, macro bodies included
#[derive(Deref, DerefMut)]
struct SceneFinder<'a>(&'a mut Vec<syn::ExprStruct>);

impl<'ast> Visit<'ast> for SceneFinder<'_> {
    fn visit_expr_struct(&mut self, node: &'ast syn::ExprStruct) {
        if scan::is_struct_literal(node, "Scene", "build") {
            self.push(node.clone());
        }
        syn::visit::visit_expr_struct(self, node);
    }

    fn visit_macro(&mut self, node: &'ast syn::Macro) {
        for expr in scan::macro_exprs(node) {
            self.visit_expr(&expr);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::scene_literals;

    #[test]
    fn test_usage() {
        let region: Vec<syn::Expr> = vec![
            syn::parse_str("Scene { name, build: preview_build, states }").unwrap(),
            syn::parse_str("vec![lele_bevy_preview::scene::Scene { build, states }]").unwrap(),
            syn::parse_str("Scene { name }").unwrap(),
        ];
        assert_eq!(scene_literals(&region).len(), 2);
    }
}
