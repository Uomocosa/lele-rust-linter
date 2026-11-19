pub fn path_segments(path: &syn::Path) -> Vec<String> {
    path.segments
        .iter()
        .map(|segment| segment.ident.to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::path_segments;
    use crate::rules;

    #[test]
    fn test_usage() {
        let path: syn::Path = syn::parse_str("crate::stock::Guest").unwrap();
        assert_eq!(
            path_segments(&path),
            vec![
                "crate".to_string(),
                "stock".to_string(),
                "Guest".to_string()
            ]
        );
        assert_eq!(
            rules::test_usage_shape::TestUsageCalls::path_segments(&path),
            vec![
                "crate".to_string(),
                "stock".to_string(),
                "Guest".to_string()
            ]
        );
    }
}
