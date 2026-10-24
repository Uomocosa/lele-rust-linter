use crate::ItemMarker;

pub fn is_rustfmt_skip(marker: ItemMarker) -> bool {
    let segments: Vec<_> = marker.path().segments.iter().collect();
    segments.len() == 2
        && segments.first().is_some_and(|s| s.ident == "rustfmt")
        && segments.get(1).is_some_and(|s| s.ident == "skip")
}

#[cfg(test)]
mod tests {
    use syn::ItemImpl;

    use crate::ItemMarker;

    #[test]
    fn test_usage() {
        let skipped: ItemImpl = syn::parse_str(
            "#[rustfmt::skip] impl Foo { pub fn new() -> Self { config_new::new() } }",
        )
        .unwrap();
        assert!(skipped
            .attrs
            .iter()
            .any(|attr| ItemMarker(attr).is_rustfmt_skip()));
        let other_attr: ItemImpl =
            syn::parse_str("#[allow(dead_code)] impl Foo { pub fn new() {} }").unwrap();
        assert!(!other_attr
            .attrs
            .iter()
            .any(|attr| ItemMarker(attr).is_rustfmt_skip()));
    }
}
