pub fn derives_component(attrs: &[syn::Attribute]) -> bool {
    attrs
        .iter()
        .filter(|attr| attr.path().is_ident("derive"))
        .any(|attr| {
            attr.parse_args_with(
                syn::punctuated::Punctuated::<syn::Path, syn::Token![,]>::parse_terminated,
            )
            .is_ok_and(|paths| {
                paths.iter().any(|path| {
                    path.segments
                        .last()
                        .is_some_and(|segment| segment.ident == "Component")
                })
            })
        })
}

#[cfg(test)]
mod tests {
    use super::derives_component;

    #[test]
    fn test_usage() {
        let component: syn::ItemStruct = syn::parse_quote! {
            #[derive(Component, Debug)]
            struct Root;
        };
        let plain: syn::ItemStruct = syn::parse_quote! {
            #[derive(Debug)]
            struct Plain;
        };
        assert!(derives_component(&component.attrs));
        assert!(!derives_component(&plain.attrs));
    }
}
