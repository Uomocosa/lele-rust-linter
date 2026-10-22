use derive_more::{Deref, DerefMut};
use syn::visit::Visit;

use crate::scan;

pub fn registered_paths(region: &scan::ExprRegion) -> Vec<Vec<String>> {
    let mut found = Vec::new();
    let mut collector = PathCollector(&mut found);
    for expr in &region.exprs {
        collector.visit_expr(expr);
    }
    found
}

// needed helper: full paths handed to `add_systems` after the schedule, chained calls included
#[derive(Deref, DerefMut)]
struct PathCollector<'a>(&'a mut Vec<Vec<String>>);

impl<'ast> Visit<'ast> for PathCollector<'_> {
    fn visit_expr_method_call(&mut self, node: &'ast syn::ExprMethodCall) {
        if node.method == "add_systems" {
            let mut paths = FullPaths(self);
            for arg in node.args.iter().skip(1) {
                paths.visit_expr(arg);
            }
        }
        syn::visit::visit_expr_method_call(self, node);
    }
}

// needed helper: every path inside one `add_systems` argument, as its segment list
#[derive(Deref, DerefMut)]
struct FullPaths<'a, 'b>(&'a mut PathCollector<'b>);

impl<'ast> Visit<'ast> for FullPaths<'_, '_> {
    fn visit_path(&mut self, node: &'ast syn::Path) {
        let segments = node
            .segments
            .iter()
            .map(|segment| segment.ident.to_string())
            .collect();
        self.push(segments);
        syn::visit::visit_path(self, node);
    }
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
