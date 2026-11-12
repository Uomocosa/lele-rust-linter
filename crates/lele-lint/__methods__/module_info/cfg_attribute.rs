pub fn cfg_attribute(attrs: &[syn::Attribute]) -> Option<String> {
    attrs.iter().find_map(|attr| {
        if !attr.path().is_ident("cfg") {
            return None;
        }
        let meta = attr.parse_args::<syn::Meta>().ok()?;
        Some(quote::quote!(#meta).to_string())
    })
}

#[cfg(test)]
mod tests {
    use super::cfg_attribute;

    #[test]
    fn test_usage() {
        let gated: syn::ItemStruct = syn::parse_quote! { #[cfg(test)] struct A; };
        assert_eq!(cfg_attribute(&gated.attrs).as_deref(), Some("test"));
        let plain: syn::ItemStruct = syn::parse_quote! { #[derive(Clone)] struct B; };
        assert_eq!(cfg_attribute(&plain.attrs), None);
    }
}
