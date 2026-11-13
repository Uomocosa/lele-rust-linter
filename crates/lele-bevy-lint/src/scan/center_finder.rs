use crate::scan;
use atomic_delegate_macros::atomic_delegate;
use derive_more::Deref;
use derive_more::DerefMut;
use syn::visit::Visit;

#[derive(Deref, DerefMut)]
pub(crate) struct CenterFinder<'a>(pub(crate) &'a mut bool);

impl<'ast> Visit<'ast> for CenterFinder<'_> {
    #[atomic_delegate(CenterFinder, Visit)]
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
