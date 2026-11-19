use crate::rules;

pub fn import_module_hint(
    prefix: &[String],
    imported: &syn::Ident,
    type_name: &str,
    anchor: &[String],
) -> Option<Vec<String>> {
    if *imported != type_name {
        return None;
    }
    rules::trait_impl_co_location::TraitImplCoLocation::defining_module_path(prefix, anchor)
}

#[cfg(test)]
mod tests {
    use super::import_module_hint;
    use crate::testing;

    #[test]
    fn test_usage() {
        let imported: syn::Ident = syn::parse_str("Config").unwrap();
        assert_eq!(
            import_module_hint(&testing::strings(&["crate", "a"]), &imported, "Config", &[]),
            Some(vec!["a".to_string(),])
        );
    }

    #[test]
    fn test_usage_skips_imports_of_other_names() {
        let imported: syn::Ident = syn::parse_str("Item").unwrap();
        assert_eq!(
            import_module_hint(&testing::strings(&["crate", "a"]), &imported, "Config", &[]),
            None
        );
    }

    #[test]
    fn test_usage_resolves_crate_root_imports_to_the_root() {
        let imported: syn::Ident = syn::parse_str("Config").unwrap();
        assert_eq!(
            import_module_hint(&testing::strings(&["crate"]), &imported, "Config", &[]),
            Some(Vec::new())
        );
    }

    #[test]
    fn test_usage_super_import_anchors_at_the_impl_file_module() {
        let imported: syn::Ident = syn::parse_str("Config").unwrap();
        assert_eq!(
            import_module_hint(
                &testing::strings(&["super", "discovery"]),
                &imported,
                "Config",
                &testing::strings(&["dial", "setup"])
            ),
            Some(vec!["dial".to_string(), "discovery".to_string()])
        );
    }

    #[test]
    fn test_usage_self_import_anchors_at_the_impl_file() {
        let imported: syn::Ident = syn::parse_str("Config").unwrap();
        assert_eq!(
            import_module_hint(
                &testing::strings(&["self", "discovery"]),
                &imported,
                "Config",
                &testing::strings(&["dial", "setup"])
            ),
            Some(vec![
                "dial".to_string(),
                "setup".to_string(),
                "discovery".to_string()
            ])
        );
    }
}

// no test_usage necessary
