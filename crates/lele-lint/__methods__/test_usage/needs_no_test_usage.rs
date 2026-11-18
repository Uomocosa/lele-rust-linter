use std::path::Path;

use crate::rules;

pub fn needs_no_test_usage(rel_path: &Path, file: &syn::File) -> bool {
    if rules::test_usage::TestUsage::is_forbidden_file(rel_path, file) {
        return true;
    }
    rel_path
        .components()
        .any(|c| c.as_os_str() == "__bevy_systems__" || c.as_os_str() == "bevy_systems")
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::needs_no_test_usage;

    #[test]
    fn test_usage() {
        let system: syn::File = syn::parse_str("pub fn tick() {}").unwrap();
        assert!(needs_no_test_usage(
            Path::new("enemy/__bevy_systems__/tick.rs"),
            &system
        ));
        assert!(needs_no_test_usage(
            Path::new("enemy/bevy_systems/tick.rs"),
            &system
        ));
        assert!(!needs_no_test_usage(Path::new("enemy/tick.rs"), &system));
        let plain: syn::File = syn::parse_quote! {
            pub struct Settings { pub port: u16 }
        };
        assert!(needs_no_test_usage(Path::new("settings.rs"), &plain));
    }
}
