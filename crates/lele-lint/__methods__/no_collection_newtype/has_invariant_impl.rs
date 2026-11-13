use crate::rules;

pub fn has_invariant_impl(items: &[syn::Item], name: &str) -> bool {
    items.iter().any(|item| {
        matches!(item, syn::Item::Impl(imp) if rules::no_collection_newtype::NoCollectionNewtype::is_invariant_impl(imp, name))
    })
}

#[cfg(test)]
mod tests {
    use crate::rules;

    #[test]
    fn test_usage() {
        let file: syn::File = syn::parse_quote! {
            pub struct Deck(Vec<u8>);
            impl TryFrom<Vec<u8>> for Deck { type Error = (); fn try_from(v: Vec<u8>) -> Result<Self, ()> { Ok(Deck(v)) } }
        };
        assert!(
            rules::no_collection_newtype::NoCollectionNewtype::has_invariant_impl(
                &file.items,
                "Deck"
            )
        );
        assert!(
            !rules::no_collection_newtype::NoCollectionNewtype::has_invariant_impl(
                &file.items,
                "Hand"
            )
        );
    }
}
