use crate::ItemMarker;

pub fn is_testing_fixture_root(item: &syn::Item) -> bool {
    let syn::Item::Mod(module) = item else {
        return false;
    };
    module.ident == "testing"
        && module.content.is_none()
        && module
            .attrs
            .iter()
            .any(|attr| ItemMarker(attr).is_cfg_test())
}

#[cfg(test)]
mod tests {
    use super::is_testing_fixture_root;
    use crate::rules;

    #[test]
    fn test_usage() {
        let root: syn::Item = syn::parse_quote! { #[cfg(test)] pub(crate) mod testing; };
        assert!(is_testing_fixture_root(&root));
        assert!(rules::test_module_items::TestModuleItems::is_testing_fixture_root(&root));
        let other: syn::Item = syn::parse_quote! { #[cfg(test)] mod tests; };
        assert!(!is_testing_fixture_root(&other));
        let inline: syn::Item = syn::parse_quote! { #[cfg(test)] mod testing {} };
        assert!(!is_testing_fixture_root(&inline));
        let plain: syn::Item = syn::parse_quote! { pub mod testing; };
        assert!(!is_testing_fixture_root(&plain));
    }
}
