pub fn cfg_test_module(item: &syn::Item) -> Option<&syn::ItemMod> {
    let syn::Item::Mod(module) = item else {
        return None;
    };
    module
        .attrs
        .iter()
        .any(|attr| lele_lint::ItemMarker(attr).is_cfg_test())
        .then_some(module)
}

#[cfg(test)]
mod tests {
    use super::cfg_test_module;

    #[test]
    fn test_usage() {
        let module: syn::Item = syn::parse_str("#[cfg(test)] mod tests {}").unwrap();
        assert!(cfg_test_module(&module).is_some());
        let plain: syn::Item = syn::parse_str("mod tests {}").unwrap();
        assert!(cfg_test_module(&plain).is_none());
        let func: syn::Item = syn::parse_str("#[cfg(test)] fn helper() {}").unwrap();
        assert!(cfg_test_module(&func).is_none());
    }
}
