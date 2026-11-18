use std::path::Path;

use crate::rules;

pub fn is_forbidden_file(rel_path: &Path, file: &syn::File) -> bool {
    let file_name = rel_path.file_name().and_then(|n| n.to_str()).unwrap_or("");
    if file_name.ends_with("_plugin.rs") {
        return false;
    }
    if file_name == "constants.rs" {
        return true;
    }
    if (file_name == "mod.rs" || file_name == "lib.rs")
        && rules::test_usage::TestUsage::is_pure_module_tree(file)
    {
        return true;
    }
    rules::test_usage::TestUsage::is_plain_data(file)
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::is_forbidden_file;

    #[test]
    fn test_usage() {
        let plain: syn::File = syn::parse_quote! {
            pub struct Settings { pub port: u16 }
        };
        assert!(is_forbidden_file(Path::new("settings.rs"), &plain));
        assert!(is_forbidden_file(Path::new("constants.rs"), &plain));
        assert!(!is_forbidden_file(Path::new("settings_plugin.rs"), &plain));
    }

    #[test]
    fn test_usage_pure_module_tree_is_forbidden() {
        let tree: syn::File = syn::parse_quote! { pub mod settings; };
        assert!(is_forbidden_file(Path::new("mod.rs"), &tree));
        let behaviour: syn::File = syn::parse_quote! {
            pub struct Guest(pub String);
            impl Guest { pub fn greet(&self) -> String { format!("hi") } }
        };
        assert!(!is_forbidden_file(Path::new("guest.rs"), &behaviour));
    }
}
