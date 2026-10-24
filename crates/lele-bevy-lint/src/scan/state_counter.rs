use derive_more::Deref;
use derive_more::DerefMut;
use syn::visit::Visit;

use crate::scan;

#[derive(Deref, DerefMut)]
pub(crate) struct StateCounter<'a>(pub(crate) &'a mut usize);

#[rustfmt::skip]
impl<'ast> Visit<'ast> for StateCounter<'_> {
    fn visit_expr_struct(&mut self, node: &'ast syn::ExprStruct) {
        let literal = scan::StructLiteral(node);
        if literal.is_type("State") && literal.has_field("apply") {
            ***self = self.saturating_add(1);
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

    use super::StateCounter;

    #[test]
    fn test_usage() {
        let expr: syn::Expr =
            syn::parse_str("vec![State { label: a, apply: before }, Other { x: 1 }]").unwrap();
        let mut count = 0_usize;
        let mut counter = StateCounter(&mut count);
        counter.visit_expr(&expr);
        assert_eq!(count, 1);
    }
}
