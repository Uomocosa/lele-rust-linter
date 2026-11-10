use crate::rules;

pub fn import_module_hint(
    prefix: &[String],
    imported: &syn::Ident,
    type_name: &str,
) -> Option<Vec<String>> {
    if *imported != type_name {
        return None;
    }
    rules::trait_impl_co_location::TraitImplCoLocation::defining_module_path(prefix)
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
            import_module_hint(&prefix(&["crate", "a"]), &imported, "Config"),
            Some(vec!["a".to_string(),])
        );
    }

    #[test]
    fn test_usage_skips_imports_of_other_names() {
        let imported: syn::Ident = syn::parse_str("Item").unwrap();
        assert_eq!(
            import_module_hint(&prefix(&["crate", "a"]), &imported, "Config"),
            None
        );
    }

    #[test]
    fn test_usage_resolves_crate_root_imports_to_the_root() {
        let imported: syn::Ident = syn::parse_str("Config").unwrap();
        assert_eq!(
            import_module_hint(&prefix(&["crate"]), &imported, "Config"),
            Some(Vec::new())
        );
    }

    #[test]
    fn test_usage_leaves_self_and_super_paths_unresolvable() {
        let imported: syn::Ident = syn::parse_str("Config").unwrap();
        assert_eq!(
            import_module_hint(&prefix(&["super", "discovery"]), &imported, "Config"),
            None
        );
    }
}

// no test_usage necessary
