use derive_more::{Deref, DerefMut};
use syn::visit::Visit;

use crate::scan;

const CENTERING_FNS: [&str; 2] = ["centered_root", "center_roots"];

#[must_use]
pub fn centers_preview(region: &[syn::Expr]) -> bool {
    let mut found = false;
    let mut finder = CenterFinder(&mut found);
    for expr in region {
        finder.visit_expr(expr);
    }
    found
}

// needed helper: flags any path ending in `centered_root`/`center_roots`, macro bodies included
#[derive(Deref, DerefMut)]
struct CenterFinder<'a>(&'a mut bool);

impl<'ast> Visit<'ast> for CenterFinder<'_> {
    fn visit_path(&mut self, node: &'ast syn::Path) {
        if node
            .segments
            .last()
            .is_some_and(|segment| CENTERING_FNS.iter().any(|name| segment.ident == name))
        {
            ***self = true;
        }
        syn::visit::visit_path(self, node);
    }

    fn visit_macro(&mut self, node: &'ast syn::Macro) {
        for expr in scan::macro_exprs(node) {
            self.visit_expr(&expr);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::centers_preview;

    #[test]
    fn test_usage() {
        let centered: Vec<syn::Expr> = vec![
            syn::parse_str("commands.spawn(lele_bevy_preview::scene::centered_root())").unwrap(),
        ];
        assert!(centers_preview(&centered));
        let chained: Vec<syn::Expr> = vec![
            syn::parse_str("app.add_systems(Startup, (spawn_root, scene::center_roots).chain())")
                .unwrap(),
        ];
        assert!(centers_preview(&chained));
        let bare: Vec<syn::Expr> =
            vec![syn::parse_str("commands.spawn((Node::default(),))").unwrap()];
        assert!(!centers_preview(&bare));
    }
}
