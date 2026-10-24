use crate::ItemMarker;

pub fn derives_any_of(marker: ItemMarker, names: &[&str]) -> bool {
    if !marker.path().is_ident("derive") {
        return false;
    }
    let syn::Meta::List(list) = &marker.meta else {
        return false;
    };
    list.tokens.to_string().split(',').any(|t| {
        let compact: String = t.split_whitespace().collect();
        names
            .iter()
            .any(|n| compact == *n || compact.ends_with(&format!("::{n}")))
    })
}

#[cfg(test)]
mod tests {
    use crate::ItemMarker;

    #[test]
    fn test_usage() {
        let pathed: syn::ItemStruct = syn::parse_quote! {
            #[derive(Debug, serde::Serialize)] struct Cfg { a: u8 }
        };
        let marker = pathed.attrs.first().map(ItemMarker).unwrap();
        assert!(marker.derives_any_of(&["Serialize"]));
        assert!(!marker.derives_any_of(&["Deref"]));
    }
}
