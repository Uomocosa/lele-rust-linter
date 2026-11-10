use lele_lint::ItemMarker;

use crate::scan;

pub fn is_test_module(item: &scan::ScannedItem) -> bool {
    let syn::Item::Mod(module) = &**item else {
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
    use crate::scan;

    #[test]
    fn test_usage() {
        let item: syn::Item = syn::parse_str("#[cfg(test)] mod tests {}").unwrap();
        assert!(is_test_module(&scan::ScannedItem(&item)));
        let plain: syn::Item = syn::parse_str("mod inner {}").unwrap();
        assert!(!is_test_module(&scan::ScannedItem(&plain)));
    }
}
