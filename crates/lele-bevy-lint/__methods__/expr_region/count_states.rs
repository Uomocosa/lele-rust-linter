use syn::visit::Visit;

use crate::scan;

pub fn count_states(region: &scan::ExprRegion) -> usize {
    let mut count = 0_usize;
    let mut counter = scan::StateCounter(&mut count);
    for expr in &region.exprs {
        counter.visit_expr(expr);
    }
    count
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
