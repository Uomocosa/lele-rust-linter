pub fn has_public_field(struct_def: &syn::ItemStruct) -> bool {
    struct_def
        .fields
        .iter()
        .any(|field| !matches!(field.vis, syn::Visibility::Inherited))
}

#[cfg(test)]
mod tests {
    use crate::rules;

    #[test]
    fn test_usage() {
        let open: syn::ItemStruct = syn::parse_quote! { pub struct Deck { pub cards: Vec<u8> } };
        assert!(rules::no_collection_newtype::NoCollectionNewtype::has_public_field(&open));
        let sealed: syn::ItemStruct = syn::parse_quote! { pub struct Deck(Vec<u8>); };
        assert!(!rules::no_collection_newtype::NoCollectionNewtype::has_public_field(&sealed));
    }
}
