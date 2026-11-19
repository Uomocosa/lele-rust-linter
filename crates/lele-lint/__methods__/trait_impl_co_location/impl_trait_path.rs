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
    use crate::testing;

    #[test]
    fn test_usage() {
        let item: syn::ItemImpl = syn::parse_str("impl Clock for UnixTime {}").unwrap();
        assert_eq!(
            super::impl_trait_path(&item),
            Some(vec!["Clock".to_string()])
        );
    }

    #[test]
    fn resolves_a_single_segment_trait_path() {
        assert_eq!(
            testing::impl_trait_path_of("impl Clock for UnixTime {}"),
            Some(vec!["Clock".to_string()])
        );
    }

    #[test]
    fn test_usage_spells_qualified_trait_paths_segment_by_segment() {
        assert_eq!(
            testing::impl_trait_path_of("impl crate::stock::Clock for UnixTime {}"),
            Some(vec![
                "crate".to_string(),
                "stock".to_string(),
                "Clock".to_string()
            ])
        );
        assert_eq!(
            testing::impl_trait_path_of("impl stock::Clock for UnixTime {}"),
            Some(vec!["stock".to_string(), "Clock".to_string()])
        );
    }

    #[test]
    fn test_usage_impls_without_a_trait_are_unresolved() {
        assert_eq!(testing::impl_trait_path_of("impl UnixTime {}"), None);
    }
}
