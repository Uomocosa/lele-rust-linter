use crate::rules;
use crate::ItemMarker;
use derive_more::{Deref, DerefMut};
use syn::visit::Visit;

#[derive(Deref, DerefMut)]
pub(crate) struct TraitVisitor<'a>(pub(crate) &'a mut Vec<rules::trait_placement::TraitHit>);

#[rustfmt::skip]
impl<'ast> Visit<'ast> for TraitVisitor<'_> {
    fn visit_item_trait(&mut self, item: &'ast syn::ItemTrait) {
        let in_test = item.attrs.iter().any(|attr| ItemMarker(attr).is_cfg_test());
        if !in_test {
            self.push(rules::trait_placement::TraitHit {
                line: item.ident.span().start().line,
                name: item.ident.to_string(),
            });
        }
        syn::visit::visit_item_trait(self, item);
    }

    fn visit_item_mod(&mut self, item: &'ast syn::ItemMod) {
        let in_test = item.attrs.iter().any(|attr| ItemMarker(attr).is_cfg_test());
        if !in_test {
            syn::visit::visit_item_mod(self, item);
        }
    }
}

// no test_usage necessary
