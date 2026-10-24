use derive_more::Deref;
use derive_more::DerefMut;
use syn::visit::Visit;

use crate::scan;

#[derive(Deref, DerefMut)]
pub(crate) struct CenterFinder<'a>(pub(crate) &'a mut bool);

#[rustfmt::skip]
impl<'ast> Visit<'ast> for CenterFinder<'_> {
    fn visit_path(&mut self, node: &'ast syn::Path) {
        if node.segments.last().is_some_and(|segment| CENTERING_FNS.iter().any(|name| segment.ident == name)) {
            ***self = true;
        }
        syn::visit::visit_path(self, node);
    }
    fn visit_macro(&mut self, node: &'ast syn::Macro) {
        for expr in scan::MacroCall(node).exprs() {
            self.visit_expr(&expr);
        }
    }
}

const CENTERING_FNS: [&str; 2] = ["centered_root", "center_roots"];

#[cfg(test)]
mod tests {
    use syn::visit::Visit;

    use super::CenterFinder;

    #[test]
    fn test_usage() {
        let expr: syn::Expr =
            syn::parse_str("commands.spawn(lele_bevy_preview::scene::centered_root())").unwrap();
        let mut found = false;
        let mut finder = CenterFinder(&mut found);
        finder.visit_expr(&expr);
        assert!(found);
    }
}
