use crate::rules;
use crate::ItemMarker;
use derive_more::{Deref, DerefMut};
use syn::spanned::Spanned;
use syn::visit::Visit;

#[derive(Deref, DerefMut)]
pub(crate) struct SuperPathVisitor<'a>(
    pub(crate) &'a mut Vec<rules::no_super_imports::SuperPathHit>,
);

impl<'ast> Visit<'ast> for SuperPathVisitor<'_> {
    fn visit_item_use(&mut self, item: &'ast syn::ItemUse) {
        if item.attrs.iter().any(|attr| ItemMarker(attr).is_cfg_test()) {
            return;
        }
        if rules::no_super_imports::NoSuperImports::starts_with_super(&item.tree) {
            self.push(rules::no_super_imports::SuperPathHit {
                line: item.span().start().line,
                path: rules::no_super_imports::NoSuperImports::use_tree_string(&item.tree),
            });
        }
    }

    fn visit_path(&mut self, path: &'ast syn::Path) {
        if path
            .segments
            .first()
            .is_some_and(|seg| seg.ident == "super")
        {
            let path_str = path
                .segments
                .iter()
                .map(|seg| seg.ident.to_string())
                .collect::<Vec<_>>()
                .join("::");
            self.push(rules::no_super_imports::SuperPathHit {
                line: path.span().start().line,
                path: path_str,
            });
        }
        syn::visit::visit_path(self, path);
    }

    fn visit_item_mod(&mut self, item: &'ast syn::ItemMod) {
        if item.attrs.iter().any(|attr| ItemMarker(attr).is_cfg_test()) {
            return;
        }
        syn::visit::visit_item_mod(self, item);
    }

    fn visit_item_fn(&mut self, item: &'ast syn::ItemFn) {
        if item.attrs.iter().any(|attr| ItemMarker(attr).is_cfg_test()) {
            return;
        }
        syn::visit::visit_item_fn(self, item);
    }
}

// no test_usage necessary
