use derive_more::{Deref, DerefMut};
use syn::visit::Visit;

use crate::scan;

const REGISTERING_METHODS: [&str; 7] = [
    "add_systems",
    "add_plugins",
    "add_observer",
    "observe",
    "register_system",
    "run_system_once",
    "run_system_cached",
];

#[must_use]
pub fn build_uses(region: &[syn::Expr]) -> Vec<String> {
    let mut uses = Vec::new();
    let mut collector = UseCollector(&mut uses);
    for expr in region {
        collector.visit_expr(expr);
    }
    uses.sort();
    uses.dedup();
    uses
}

// needed helper: names that are called, or handed to a registering App/World method
#[derive(Deref, DerefMut)]
struct UseCollector<'a>(&'a mut Vec<String>);

impl<'ast> Visit<'ast> for UseCollector<'_> {
    fn visit_expr_call(&mut self, node: &'ast syn::ExprCall) {
        if let syn::Expr::Path(path) = &*node.func
            && let Some(last) = path.path.segments.last()
        {
            self.push(last.ident.to_string());
        }
        syn::visit::visit_expr_call(self, node);
    }

    fn visit_expr_method_call(&mut self, node: &'ast syn::ExprMethodCall) {
        if REGISTERING_METHODS
            .iter()
            .any(|method| node.method == method)
        {
            let mut names = RegisteredNames(self);
            for arg in &node.args {
                names.visit_expr(arg);
            }
        }
        syn::visit::visit_expr_method_call(self, node);
    }

    fn visit_macro(&mut self, node: &'ast syn::Macro) {
        for expr in scan::macro_exprs(node) {
            self.visit_expr(&expr);
        }
    }
}

// needed helper: the last segment of every path inside a registering call's arguments
#[derive(Deref, DerefMut)]
struct RegisteredNames<'a, 'b>(&'a mut UseCollector<'b>);

impl<'ast> Visit<'ast> for RegisteredNames<'_, '_> {
    fn visit_path(&mut self, node: &'ast syn::Path) {
        if let Some(last) = node.segments.last() {
            self.push(last.ident.to_string());
        }
        syn::visit::visit_path(self, node);
    }
}

#[cfg(test)]
mod tests {
    use super::build_uses;

    #[test]
    fn test_usage() {
        let region: Vec<syn::Expr> = vec![syn::parse_str(
            "{
                 app.add_systems(Startup, super::spawn_root).add_plugins(discovery::ui::DefaultUiPlugin);
                 helper(app);
                 let _ = super::setup_ui;
             }",
        )
        .unwrap()];
        let uses = build_uses(&region);
        assert!(uses.contains(&"spawn_root".to_string()));
        assert!(uses.contains(&"DefaultUiPlugin".to_string()));
        assert!(uses.contains(&"helper".to_string()));
        assert!(!uses.contains(&"setup_ui".to_string()));
    }
}
