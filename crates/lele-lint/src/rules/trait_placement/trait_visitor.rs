use crate::rules;
use atomic_delegate_macros::atomic_delegates;
use derive_more::{Deref, DerefMut};
use syn::visit::Visit;

#[derive(Deref, DerefMut)]
pub(crate) struct TraitVisitor<'a>(pub(crate) &'a mut Vec<rules::trait_placement::TraitHit>);

#[atomic_delegates]
impl<'ast> Visit<'ast> for TraitVisitor<'_> {
    fn visit_item_trait(&mut self, item: &'ast syn::ItemTrait) {}
    fn visit_item_mod(&mut self, item: &'ast syn::ItemMod) {}
}

// no test_usage necessary
