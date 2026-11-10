use syn::visit::Visit;

use crate::scan;

pub fn centers_preview(region: &scan::ExprRegion) -> bool {
    let mut found = false;
    let mut finder = scan::CenterFinder(&mut found);
    for expr in &region.exprs {
        finder.visit_expr(expr);
    }
    found
}

#[cfg(test)]
mod tests {
    use super::centers_preview;
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
        let start: syn::Expr =
            syn::parse_str("commands.spawn(lele_bevy_preview::scene::centered_root())").unwrap();
        assert!(centers_preview(&scan::ExprRegion::over(start, &module_fns)));
    }
}
