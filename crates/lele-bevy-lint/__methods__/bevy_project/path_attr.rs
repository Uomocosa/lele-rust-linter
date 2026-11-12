pub fn path_attr(attrs: &[syn::Attribute]) -> Option<String> {
    attrs.iter().find_map(|attr| {
        let syn::Meta::NameValue(pair) = &attr.meta else {
            return None;
        };
        if !pair.path.is_ident("path") {
            return None;
        }
        let syn::Expr::Lit(syn::ExprLit {
            lit: syn::Lit::Str(value),
            ..
        }) = &pair.value
        else {
            return None;
        };
        Some(value.value())
    })
}

#[cfg(test)]
mod tests {
    use super::path_attr;

    #[test]
    fn test_usage() {
        let mounted: syn::ItemMod = syn::parse_quote! {
            #[path = "__basic__/mod.rs"]
            mod basic;
        };
        let plain: syn::ItemMod = syn::parse_quote! {
            mod plain;
        };
        assert_eq!(
            path_attr(&mounted.attrs),
            Some("__basic__/mod.rs".to_string())
        );
        assert_eq!(path_attr(&plain.attrs), None);
    }
}
