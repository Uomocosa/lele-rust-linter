use crate::rules;

pub fn is_invariant_impl(imp: &syn::ItemImpl, name: &str) -> bool {
    if !rules::no_collection_newtype::NoCollectionNewtype::impl_self_is(imp, name) {
        return false;
    }
    match &imp.trait_ {
        None => imp
            .items
            .iter()
            .any(|item| matches!(item, syn::ImplItem::Fn(_))),
        Some((_, path, _)) => path.segments.last().is_some_and(|segment| {
            matches!(segment.ident.to_string().as_str(), "From" | "TryFrom")
        }),
    }
}

#[cfg(test)]
mod tests {
    use crate::rules;

    #[test]
    fn test_usage() {
        let inherent: syn::ItemImpl = syn::parse_quote!(impl Deck { pub fn size(&self) {} });
        let conversion: syn::ItemImpl = syn::parse_quote!(impl TryFrom<Vec<Card>> for Deck {});
        let other: syn::ItemImpl = syn::parse_quote!(impl From<Deck> for Vec<Card> {});
        assert!(
            rules::no_collection_newtype::NoCollectionNewtype::is_invariant_impl(&inherent, "Deck")
        );
        assert!(
            rules::no_collection_newtype::NoCollectionNewtype::is_invariant_impl(
                &conversion,
                "Deck"
            )
        );
        assert!(
            !rules::no_collection_newtype::NoCollectionNewtype::is_invariant_impl(&other, "Deck")
        );
    }
}
