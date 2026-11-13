use crate::ItemMarker;

pub fn derives_deref_mut(struct_def: &syn::ItemStruct) -> bool {
    struct_def
        .attrs
        .iter()
        .any(|attr| ItemMarker(attr).derives_any_of(&["DerefMut"]))
}

#[cfg(test)]
mod tests {
    use crate::rules;

    #[test]
    fn test_usage() {
        let with_mut: syn::ItemStruct =
            syn::parse_quote! { #[derive(Deref, DerefMut)] pub struct Deck(Vec<u8>); };
        assert!(rules::no_collection_newtype::NoCollectionNewtype::derives_deref_mut(&with_mut));
        let without_mut: syn::ItemStruct =
            syn::parse_quote! { #[derive(Deref)] pub struct Deck(Vec<u8>); };
        assert!(
            !rules::no_collection_newtype::NoCollectionNewtype::derives_deref_mut(&without_mut)
        );
    }
}
