pub fn impl_trait_path(item: &syn::ItemImpl) -> Option<Vec<String>> {
    let Some((_, trait_path, _)) = &item.trait_ else {
        return None;
    };
    Some(
        trait_path
            .segments
            .iter()
            .map(|segment| segment.ident.to_string())
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::impl_trait_path;

    fn trait_path_of(impl_source: &str) -> Option<Vec<String>> {
        let impl_block: syn::ItemImpl = syn::parse_str(impl_source).unwrap();
        impl_trait_path(&impl_block)
    }

    #[test]
    fn test_usage() {
        assert_eq!(
            trait_path_of("impl Clock for UnixTime {}"),
            Some(vec!["Clock".to_string()])
        );
    }

    #[test]
    fn test_usage_spells_qualified_trait_paths_segment_by_segment() {
        assert_eq!(
            trait_path_of("impl crate::stock::Clock for UnixTime {}"),
            Some(vec![
                "crate".to_string(),
                "stock".to_string(),
                "Clock".to_string()
            ])
        );
        assert_eq!(
            trait_path_of("impl stock::Clock for UnixTime {}"),
            Some(vec!["stock".to_string(), "Clock".to_string()])
        );
    }

    #[test]
    fn test_usage_impls_without_a_trait_are_unresolved() {
        assert_eq!(trait_path_of("impl UnixTime {}"), None);
    }
}
