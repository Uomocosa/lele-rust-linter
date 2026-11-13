use atomic_delegate_macros::atomic_delegates;
use derive_more::Deref;
use derive_more::DerefMut;
use syn::visit::Visit;

#[derive(Deref, DerefMut)]
pub(crate) struct PathCollector<'a>(pub(crate) &'a mut Vec<Vec<String>>);

#[atomic_delegates]
impl<'ast> Visit<'ast> for PathCollector<'_> {
    fn visit_expr_method_call(&mut self, node: &'ast syn::ExprMethodCall) {}
}

#[cfg(test)]
mod tests {
    use syn::visit::Visit;

    use super::PathCollector;

    #[test]
    fn test_usage() {
        let expr: syn::Expr =
            syn::parse_str("app.add_systems(Startup, discovery::ui::bevy_systems::spawn_root)")
                .unwrap();
        let mut found = Vec::new();
        let mut collector = PathCollector(&mut found);
        collector.visit_expr(&expr);
        assert!(found.contains(&vec![
            "discovery".to_string(),
            "ui".to_string(),
            "bevy_systems".to_string(),
            "spawn_root".to_string(),
        ]));
        assert!(!found.contains(&vec!["Startup".to_string()]));
    }
}
