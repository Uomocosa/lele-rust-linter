use syn::visit::Visit;

use crate::scan;

pub fn build_uses(region: &scan::ExprRegion) -> Vec<String> {
    let mut uses = Vec::new();
    let mut collector = scan::UseCollector(&mut uses);
    for expr in &region.exprs {
        collector.visit_expr(expr);
    }
    uses.sort();
    uses.dedup();
    uses
}

#[cfg(test)]
mod tests {
    use super::build_uses;
    use crate::scan;

    #[test]
    fn test_usage() {
        let file: syn::File = syn::parse_str(
            "fn build(app: &mut App) {
                 app.add_systems(Startup, super::spawn_root).add_plugins(discovery::ui::DefaultUiPlugin);
                 helper(app);
                 let _ = super::setup_ui;
             }",
        )
        .unwrap();
        let module_fns: scan::ModuleFns = file
            .items
            .iter()
            .filter_map(|item| match item {
                syn::Item::Fn(func) => Some((func.sig.ident.to_string(), func)),
                _ => None,
            })
            .collect();
        let start: syn::Expr = syn::parse_str("build(app)").unwrap();
        let uses = build_uses(&scan::ExprRegion::over(start, &module_fns));
        assert!(uses.contains(&"spawn_root".to_string()));
        assert!(uses.contains(&"DefaultUiPlugin".to_string()));
        assert!(uses.contains(&"helper".to_string()));
        assert!(!uses.contains(&"setup_ui".to_string()));
    }
}
