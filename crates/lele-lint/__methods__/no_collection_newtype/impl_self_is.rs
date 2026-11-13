pub fn impl_self_is(imp: &syn::ItemImpl, name: &str) -> bool {
    matches!(&*imp.self_ty, syn::Type::Path(path) if path.path.segments.last().is_some_and(|segment| segment.ident == name))
}

#[cfg(test)]
mod tests {
    use crate::rules;

    #[test]
    fn test_usage() {
        let block: syn::ItemImpl = syn::parse_quote! { impl Deck {} };
        assert!(rules::no_collection_newtype::NoCollectionNewtype::impl_self_is(&block, "Deck"));
        assert!(!rules::no_collection_newtype::NoCollectionNewtype::impl_self_is(&block, "Hand"));
    }
}
