use syn::visit::Visit;

use crate::scan;

pub fn scene_literals(region: &scan::ExprRegion) -> Vec<syn::ExprStruct> {
    let mut found = Vec::new();
    let mut finder = scan::SceneFinder(&mut found);
    for expr in &region.exprs {
        finder.visit_expr(expr);
    }
    found
}

#[cfg(test)]
mod tests {
    use super::scene_literals;
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
            syn::parse_str("Scene { name, build: preview_build, states }").unwrap();
        assert_eq!(
            scene_literals(&scan::ExprRegion::over(start, &module_fns)).len(),
            1
        );
    }
}
