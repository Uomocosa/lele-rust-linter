pub fn trait_path_matches(trait_path: &str, impl_trait: &syn::Path) -> bool {
    let entry_segments: Vec<&str> = trait_path.split("::").collect();
    let written: Vec<String> = impl_trait
        .segments
        .iter()
        .map(|segment| segment.ident.to_string())
        .collect();
    let (Some(entry_last), Some(written_last)) = (entry_segments.last(), written.last()) else {
        return false;
    };
    if *entry_last != written_last.as_str() {
        return false;
    }
    written
        .iter()
        .filter(|segment| segment.as_str() != "prelude")
        .all(|segment| entry_segments.contains(&segment.as_str()))
}

#[cfg(test)]
mod tests {
    use crate::rules;

    #[test]
    fn test_usage() {
        let matches = |path: &str, written: &str| {
            let impl_trait: syn::Path = syn::parse_str(written).unwrap();
            rules::inline_trait_methods::InlineTraitMethods::trait_path_matches(path, &impl_trait)
        };
        assert!(matches("bevy::app::Plugin", "Plugin"));
        assert!(matches("bevy::app::Plugin", "bevy::prelude::Plugin"));
        assert!(matches("bevy::app::Plugin", "bevy::app::Plugin"));
        assert!(!matches("bevy::app::Plugin", "other::Plugin"));
        assert!(!matches("bevy::app::Plugin", "Rule"));
    }
}
