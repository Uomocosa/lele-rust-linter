pub fn impl_type_path(item: &syn::ItemImpl) -> Option<Vec<String>> {
    let mut self_type = &*item.self_ty;
    loop {
        match self_type {
            syn::Type::Path(type_path) => {
                return Some(
                    type_path
                        .path
                        .segments
                        .iter()
                        .map(|segment| segment.ident.to_string())
                        .collect(),
                );
            }
            syn::Type::Reference(reference) => self_type = &reference.elem,
            syn::Type::Paren(paren) => self_type = &paren.elem,
            _ => return None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::impl_type_path;

    fn path_of(impl_source: &str) -> Option<Vec<String>> {
        let impl_block: syn::ItemImpl = syn::parse_str(impl_source).unwrap();
        impl_type_path(&impl_block)
    }

    #[test]
    fn test_usage() {
        assert_eq!(
            path_of("impl crate::stock::Clock for UnixTime {}"),
            Some(vec!["UnixTime".to_string()])
        );
    }

    #[test]
    fn test_usage_spells_qualified_self_paths_segment_by_segment() {
        assert_eq!(
            path_of("impl Clock for crate::stock::UnixTime {}"),
            Some(vec![
                "crate".to_string(),
                "stock".to_string(),
                "UnixTime".to_string()
            ])
        );
        assert_eq!(
            path_of("impl Clock for stock::UnixTime {}"),
            Some(vec!["stock".to_string(), "UnixTime".to_string()])
        );
    }

    #[test]
    fn test_usage_unwraps_references_and_parens() {
        assert_eq!(
            path_of("impl Clock for &UnixTime {}"),
            Some(vec!["UnixTime".to_string()])
        );
        assert_eq!(
            path_of("impl Clock for &&(UnixTime) {}"),
            Some(vec!["UnixTime".to_string()])
        );
    }

    #[test]
    fn test_usage_leaves_slices_and_tuples_unresolved() {
        assert_eq!(path_of("impl Clock for [UnixTime] {}"), None);
        assert_eq!(path_of("impl Clock for (UnixTime, UnixTime) {}"), None);
    }
}

// no test_usage necessary
