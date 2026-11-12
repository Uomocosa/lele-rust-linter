use crate::ItemMarker;

pub fn derives_data_shape(item_marker: ItemMarker) -> bool {
    item_marker.derives_any_of(&[
        "Serialize",
        "Deserialize",
        "Parser",
        "Args",
        "Subcommand",
        "ValueEnum",
    ])
}

#[cfg(test)]
mod tests {
    use super::derives_data_shape;
    use crate::ItemMarker;

    #[test]
    fn test_usage() {
        let wire: syn::ItemStruct =
            syn::parse_quote! { #[derive(Parser)] struct Args { root: String } };
        assert!(derives_data_shape(ItemMarker(&wire.attrs[0])));
        let plain: syn::ItemStruct = syn::parse_quote! { #[derive(Clone)] struct X { value: u64 } };
        assert!(!derives_data_shape(ItemMarker(&plain.attrs[0])));
    }
}
