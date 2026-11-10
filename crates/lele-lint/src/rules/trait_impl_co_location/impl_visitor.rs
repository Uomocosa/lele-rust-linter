use crate::rules;
use crate::ImplBlock;
use crate::ItemMarker;
use derive_more::{Deref, DerefMut};
use syn::visit::Visit;

#[derive(Deref, DerefMut)]
pub(crate) struct ImplVisitor<'a>(pub(crate) &'a mut Vec<rules::trait_impl_co_location::ImplHit>);

impl<'ast> Visit<'ast> for ImplVisitor<'_> {
    fn visit_item_impl(&mut self, item: &'ast syn::ItemImpl) {
        let in_test = item.attrs.iter().any(|attr| ItemMarker(attr).is_cfg_test());
        if !in_test {
            if let Some((_, trait_path, _)) = &item.trait_ {
                if let Some(trait_name) = trait_path
                    .segments
                    .last()
                    .map(|segment| segment.ident.to_string())
                {
                    self.push(rules::trait_impl_co_location::ImplHit {
                        line: item.impl_token.span.start().line,
                        trait_name,
                        type_name: ImplBlock(item).self_type_last(),
                        is_blanket: ImplVisitor::is_blanket_self(item),
                    });
                }
            }
        }
        syn::visit::visit_item_impl(self, item);
    }

    fn visit_item_mod(&mut self, item: &'ast syn::ItemMod) {
        let in_test = item.attrs.iter().any(|attr| ItemMarker(attr).is_cfg_test());
        if !in_test {
            syn::visit::visit_item_mod(self, item);
        }
    }
}

#[rustfmt::skip]
impl ImplVisitor<'_> {
    pub(crate) fn is_blanket_self(item: &syn::ItemImpl) -> bool {
        matches!(&*item.self_ty, syn::Type::Path(type_path)
            if type_path.path.get_ident().is_some_and(|ident| {
                item.generics.type_params().any(|param| param.ident == *ident)
            }))
    }
}

// no test_usage necessary
