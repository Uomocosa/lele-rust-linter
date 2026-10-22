use derive_more::{Deref, DerefMut};
use syn::visit::Visit;

use crate::scan;

pub fn centers_preview(region: &scan::ExprRegion) -> bool {
    let mut found = false;
    let mut finder = CenterFinder(&mut found);
    for expr in &region.exprs {
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
        for expr in scan::MacroCall(node).exprs() {
            self.visit_expr(&expr);
        }
    }
}

const CENTERING_FNS: [&str; 2] = ["centered_root", "center_roots"];

#[cfg(test)]
mod tests {
    use super::centers_preview;
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
        let start: syn::Expr =
            syn::parse_str("commands.spawn(lele_bevy_preview::scene::centered_root())").unwrap();
        assert!(centers_preview(&scan::ExprRegion::over(start, &module_fns)));
    }
}
