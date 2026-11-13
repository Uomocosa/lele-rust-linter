use crate::ItemMarker;

pub fn has_deref_derive(struct_def: &syn::ItemStruct) -> bool {
    struct_def
        .attrs
        .iter()
        .any(|attr| ItemMarker(attr).derives_any_of(&["Deref"]))
}

#[cfg(test)]
mod tests {
    use crate::rules;

    #[test]
    fn test_usage() {
        let derived: syn::ItemStruct =
            syn::parse_quote! { #[derive(Deref)] pub struct Meters(f64); };
        assert!(rules::single_field_newtype::SingleFieldNewtype::has_deref_derive(&derived));
        let plain: syn::ItemStruct = syn::parse_quote! { #[derive(Debug)] pub struct Meters(f64); };
        assert!(!rules::single_field_newtype::SingleFieldNewtype::has_deref_derive(&plain));
    }
}
