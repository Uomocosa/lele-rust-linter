use derive_more::{Deref, DerefMut};
use syn::visit::Visit;

use crate::scan;

#[must_use]
pub fn count_states(region: &[syn::Expr]) -> usize {
    let mut count = 0_usize;
    let mut counter = StateCounter(&mut count);
    for expr in region {
        counter.visit_expr(expr);
    }
    count
}

// needed helper: counts `State { apply, .. }` literals, macro bodies included
#[derive(Deref, DerefMut)]
struct StateCounter<'a>(&'a mut usize);

impl<'ast> Visit<'ast> for StateCounter<'_> {
    fn visit_expr_struct(&mut self, node: &'ast syn::ExprStruct) {
        if scan::is_struct_literal(node, "State", "apply") {
            ***self = self.saturating_add(1);
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
    use super::count_states;

    #[test]
    fn test_usage() {
        let region: Vec<syn::Expr> = vec![
            syn::parse_str(
                "vec![State { label: a, apply: before }, scene::State { label: b, apply: after }]",
            )
            .unwrap(),
        ];
        assert_eq!(count_states(&region), 2);
        assert_eq!(count_states(&[syn::parse_str("vec![]").unwrap()]), 0);
    }
}
