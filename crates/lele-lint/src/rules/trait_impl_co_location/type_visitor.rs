use crate::ItemMarker;
use derive_more::{Deref, DerefMut};
use syn::visit::Visit;

#[derive(Deref, DerefMut)]
pub(crate) struct TypeVisitor<'a>(pub(crate) &'a mut Vec<String>);

impl<'ast> Visit<'ast> for TypeVisitor<'_> {
    fn visit_item_struct(&mut self, item: &'ast syn::ItemStruct) {
        self.record(&item.attrs, &item.ident);
        syn::visit::visit_item_struct(self, item);
    }

    fn visit_item_enum(&mut self, item: &'ast syn::ItemEnum) {
        self.record(&item.attrs, &item.ident);
        syn::visit::visit_item_enum(self, item);
    }

    fn visit_item_union(&mut self, item: &'ast syn::ItemUnion) {
        self.record(&item.attrs, &item.ident);
        syn::visit::visit_item_union(self, item);
    }

    fn visit_item_type(&mut self, item: &'ast syn::ItemType) {
        self.record(&item.attrs, &item.ident);
        syn::visit::visit_item_type(self, item);
    }

    fn visit_item_mod(&mut self, item: &'ast syn::ItemMod) {
        let in_test = item.attrs.iter().any(|attr| ItemMarker(attr).is_cfg_test());
        if !in_test {
            syn::visit::visit_item_mod(self, item);
        }
    }
}

impl TypeVisitor<'_> {
    pub(crate) fn record(&mut self, attrs: &[syn::Attribute], ident: &syn::Ident) {
        if !attrs.iter().any(|attr| ItemMarker(attr).is_cfg_test()) {
            self.push(ident.to_string());
        }
    }
}

// no test_usage necessary
