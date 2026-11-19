use crate::ItemMarker;

pub fn is_test_module(item: &syn::Item) -> bool {
    let syn::Item::Mod(module) = item else {
        return false;
    };
    module
        .attrs
        .iter()
        .any(|attr| ItemMarker(attr).is_cfg_test())
}

#[cfg(test)]
mod tests {
    use super::is_test_module;
    use crate::rules;

    #[test]
    fn test_usage() {
        let module: syn::Item = syn::parse_quote! { #[cfg(test)] mod tests {} };
        assert!(is_test_module(&module));
        assert!(rules::test_module_items::TestModuleItems::is_test_module(
            &module
        ));
        let aligned: syn::Item = syn::parse_quote! { #[cfg(all(test, unix))] mod tests {} };
        assert!(is_test_module(&aligned));
        let not_test: syn::Item = syn::parse_quote! { #[cfg(not(test))] mod tests {} };
        assert!(!is_test_module(&not_test));
        let item: syn::Item = syn::parse_quote! { pub struct Holder; };
        assert!(!is_test_module(&item));
    }
}
