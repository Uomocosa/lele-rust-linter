use syn::visit::Visit;

use crate::scan;

pub fn registered_paths(region: &scan::ExprRegion) -> Vec<Vec<String>> {
    let mut found = Vec::new();
    let mut collector = scan::PathCollector(&mut found);
    for expr in &region.exprs {
        collector.visit_expr(expr);
    }
    found
}

#[cfg(test)]
mod tests {
    use super::registered_paths;
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
            "app.add_systems(Startup, discovery::ui::bevy_systems::spawn_root)
                 .add_systems(Update, (tick, gate).chain())",
        )
        .unwrap();
        let paths = registered_paths(&scan::ExprRegion::over(start, &module_fns));
        assert!(paths.contains(&vec![
            "discovery".to_string(),
            "ui".to_string(),
            "bevy_systems".to_string(),
            "spawn_root".to_string(),
        ]));
        assert!(paths.contains(&vec!["tick".to_string()]));
        assert!(!paths.contains(&vec!["Startup".to_string()]));
    }
}
