use crate::scan;
use atomic_delegate_macros::atomic_delegate;
use derive_more::Deref;
use derive_more::DerefMut;
use syn::visit::Visit;

#[derive(Deref, DerefMut)]
pub(crate) struct PathNames<'a>(pub(crate) &'a mut Vec<String>);

impl<'ast> Visit<'ast> for PathNames<'_> {
    #[atomic_delegate(PathNames, Visit)]
    fn visit_path(&mut self, node: &'ast syn::Path) {}
    fn visit_macro(&mut self, node: &'ast syn::Macro) {
        for expr in scan::MacroCall(node).exprs() {
            self.visit_expr(&expr);
        }
    }
}

#[cfg(test)]
mod tests {
    use syn::visit::Visit;

    use super::PathNames;

    #[test]
    fn test_usage() {
        let expr: syn::Expr = syn::parse_str("spawn((Node, Text::new(\"x\")))").unwrap();
        let mut collected = Vec::new();
        let mut names = PathNames(&mut collected);
        names.visit_expr(&expr);
        assert!(collected.contains(&"Node".to_string()));
    }
}
