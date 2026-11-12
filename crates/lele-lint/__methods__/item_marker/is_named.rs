use crate::ItemMarker;

pub fn is_named(item_marker: ItemMarker, name: &str) -> bool {
    item_marker
        .path()
        .segments
        .last()
        .is_some_and(|segment| segment.ident == name)
}

#[cfg(test)]
mod tests {
    use super::is_named;
    use crate::ItemMarker;

    #[test]
    fn test_usage() {
        let item: syn::ItemStruct = syn::parse_quote! { #[serde(default)] struct A; };
        let marker = item.attrs.first().map(ItemMarker).unwrap();
        assert!(is_named(marker, "serde"));
        assert!(!is_named(marker, "test"));
    }
}
