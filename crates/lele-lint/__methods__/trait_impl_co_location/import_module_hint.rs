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

    fn prefix(names: &[&str]) -> Vec<String> {
        names.iter().map(|name| name.to_string()).collect()
    }

    #[test]
    fn test_usage() {
        let imported: syn::Ident = syn::parse_str("Config").unwrap();
        assert_eq!(
            import_module_hint(&prefix(&["crate", "a"]), &imported, "Config", &[]),
            Some(vec!["a".to_string(),])
        );
    }

    #[test]
    fn test_usage_skips_imports_of_other_names() {
        let imported: syn::Ident = syn::parse_str("Item").unwrap();
        assert_eq!(
            import_module_hint(&prefix(&["crate", "a"]), &imported, "Config", &[]),
            None
        );
    }

    #[test]
    fn test_usage_resolves_crate_root_imports_to_the_root() {
        let imported: syn::Ident = syn::parse_str("Config").unwrap();
        assert_eq!(
            import_module_hint(&prefix(&["crate"]), &imported, "Config", &[]),
            Some(Vec::new())
        );
    }

    #[test]
    fn test_usage_super_import_anchors_at_the_impl_file_module() {
        let imported: syn::Ident = syn::parse_str("Config").unwrap();
        assert_eq!(
            import_module_hint(
                &prefix(&["super", "discovery"]),
                &imported,
                "Config",
                &prefix(&["dial", "setup"])
            ),
            Some(vec!["dial".to_string(), "discovery".to_string()])
        );
    }

    #[test]
    fn test_usage_self_import_anchors_at_the_impl_file() {
        let imported: syn::Ident = syn::parse_str("Config").unwrap();
        assert_eq!(
            import_module_hint(
                &prefix(&["self", "discovery"]),
                &imported,
                "Config",
                &prefix(&["dial", "setup"])
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
