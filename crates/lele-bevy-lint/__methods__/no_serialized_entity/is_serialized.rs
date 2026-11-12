use lele_lint::ItemMarker;

pub fn is_serialized(attrs: &[syn::Attribute]) -> bool {
    attrs
        .iter()
        .any(|attr| ItemMarker(attr).derives_any_of(&["Serialize", "Deserialize"]))
}

#[cfg(test)]
mod tests {
    use super::is_serialized;

    #[test]
    fn test_usage() {
        let serialized: syn::ItemStruct = syn::parse_quote! {
            #[derive(Serialize)]
            struct Snapshot;
        };
        let plain: syn::ItemStruct = syn::parse_quote! {
            #[derive(Debug)]
            struct Live;
        };
        assert!(is_serialized(&serialized.attrs));
        assert!(!is_serialized(&plain.attrs));
    }
}
