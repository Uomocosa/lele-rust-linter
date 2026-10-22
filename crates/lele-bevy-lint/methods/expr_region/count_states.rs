use derive_more::{Deref, DerefMut};
use syn::visit::Visit;

use crate::scan;

pub fn count_states(region: &scan::ExprRegion) -> usize {
    let mut count = 0_usize;
    let mut counter = StateCounter(&mut count);
    for expr in &region.exprs {
        counter.visit_expr(expr);
    }
    count
}

// needed helper: counts `State { apply, .. }` literals, macro bodies included
#[derive(Deref, DerefMut)]
struct StateCounter<'a>(&'a mut usize);

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
    use super::count_states;
    use crate::scan;

    #[test]
    fn test_usage() {
        let file: syn::File = syn::parse_str("fn build(app: &mut App) {}").unwrap();
        let module_fns: scan::ModuleFns = file
            .items
            .iter()
            .filter_map(|item| match item {
                syn::Item::Fn(func) => Some((func.sig.ident.to_string(), func)),
                _ => None,
            })
            .collect();
        let start: syn::Expr = syn::parse_str(
            "vec![State { label: a, apply: before }, scene::State { label: b, apply: after }]",
        )
        .unwrap();
        assert_eq!(count_states(&scan::ExprRegion::over(start, &module_fns)), 2);
    }
}
