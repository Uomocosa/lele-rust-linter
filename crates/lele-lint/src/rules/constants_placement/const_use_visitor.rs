use crate::ItemMarker;
use std::collections::HashSet;
use syn::visit::Visit;

pub(crate) struct ConstUseVisitor<'a> {
    pub(crate) defined: &'a HashSet<String>,
    pub(crate) found: Vec<Vec<String>>,
}

impl<'ast> Visit<'ast> for ConstUseVisitor<'_> {
    fn visit_item_mod(&mut self, item: &'ast syn::ItemMod) {
        if item.attrs.iter().any(|attr| ItemMarker(attr).is_cfg_test()) {
            return;
        }
        syn::visit::visit_item_mod(self, item);
    }

    fn visit_path(&mut self, path: &'ast syn::Path) {
        if let Some(last) = path.segments.last() {
            if self.defined.contains(&last.ident.to_string()) {
                self.found.push(
                    path.segments
                        .iter()
                        .map(|segment| segment.ident.to_string())
                        .collect(),
                );
            }
        }
        syn::visit::visit_path(self, path);
    }
}

// no test_usage necessary
