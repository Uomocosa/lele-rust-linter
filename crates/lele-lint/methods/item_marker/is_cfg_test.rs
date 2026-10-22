use syn::meta::ParseNestedMeta;

use crate::ItemMarker;

pub fn is_cfg_test(marker: &ItemMarker) -> bool {
    if !marker.path().is_ident("cfg") {
        return false;
    }
    let syn::Meta::List(list) = &marker.meta else {
        return false;
    };
    list.parse_nested_meta(|meta| {
        if meta.path.is_ident("test") {
            return Ok(());
        }
        if meta.path.is_ident("all") {
            return all_contains_test(&meta);
        }
        Err(meta.error("expected test"))
    })
    .is_ok()
}

// needed helper: `all(test, ...)` gates the item on test as well
fn all_contains_test(meta: &ParseNestedMeta) -> syn::Result<()> {
    let mut found = false;
    let _ = meta.parse_nested_meta(|inner| {
        if inner.path.is_ident("test") {
            found = true;
        }
        Ok(())
    });
    if found {
        Ok(())
    } else {
        Err(meta.error("expected test"))
    }
}

#[cfg(test)]
mod tests {
    use crate::ItemMarker;

    #[test]
    fn test_usage() {
        let test_impl: syn::ItemImpl =
            syn::parse_quote! { #[cfg(test)] impl Foo { fn help(&self) {} } };
        assert!(test_impl
            .attrs
            .iter()
            .any(|attr| ItemMarker(attr).is_cfg_test()));
        let other_cfg: syn::ItemImpl =
            syn::parse_quote! { #[cfg(unix)] impl Foo { fn help(&self) {} } };
        assert!(!other_cfg
            .attrs
            .iter()
            .any(|attr| ItemMarker(attr).is_cfg_test()));
        let module: syn::ItemMod = syn::parse_str("#[cfg(test)] mod tests {}").unwrap();
        assert!(module
            .attrs
            .iter()
            .any(|attr| ItemMarker(attr).is_cfg_test()));
    }

    #[test]
    fn test_usage_all_with_test_is_a_test_cfg() {
        let module: syn::ItemMod = syn::parse_str("#[cfg(all(test, unix))] mod tests {}").unwrap();
        assert!(module
            .attrs
            .iter()
            .any(|attr| ItemMarker(attr).is_cfg_test()));
    }

    #[test]
    fn test_usage_not_test_is_not_a_test_cfg() {
        let module: syn::ItemMod = syn::parse_str("#[cfg(not(test))] mod tests {}").unwrap();
        assert!(!module
            .attrs
            .iter()
            .any(|attr| ItemMarker(attr).is_cfg_test()));
    }
}
