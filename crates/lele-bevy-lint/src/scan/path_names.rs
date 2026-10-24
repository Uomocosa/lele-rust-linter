use derive_more::Deref;
use derive_more::DerefMut;
use syn::visit::Visit;

use crate::scan;

#[derive(Deref, DerefMut)]
pub(crate) struct PathNames<'a>(pub(crate) &'a mut Vec<String>);

#[rustfmt::skip]
impl<'ast> Visit<'ast> for PathNames<'_> {
    fn visit_path(&mut self, node: &'ast syn::Path) {
        for segment in &node.segments {
            self.push(segment.ident.to_string());
        }
        syn::visit::visit_path(self, node);
    }
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
