use derive_more::Deref;
use derive_more::DerefMut;
use syn::visit::Visit;

use crate::scan;

#[derive(Deref, DerefMut)]
pub(crate) struct SceneFinder<'a>(pub(crate) &'a mut Vec<syn::ExprStruct>);

#[rustfmt::skip]
impl<'ast> Visit<'ast> for SceneFinder<'_> {
    fn visit_expr_struct(&mut self, node: &'ast syn::ExprStruct) {
        let literal = scan::StructLiteral(node);
        if literal.is_type("Scene") && literal.has_field("build") {
            self.push(node.clone());
        }
        syn::visit::visit_expr_struct(self, node);
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

    use super::SceneFinder;

    #[test]
    fn test_usage() {
        let expr: syn::Expr =
            syn::parse_str("Scene { name, build: preview_build, states }").unwrap();
        let mut found = Vec::new();
        let mut finder = SceneFinder(&mut found);
        finder.visit_expr(&expr);
        assert_eq!(found.len(), 1);
    }
}
