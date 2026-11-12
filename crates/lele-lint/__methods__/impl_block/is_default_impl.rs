use crate::ImplBlock;

pub fn is_default_impl(impl_block: ImplBlock) -> bool {
    impl_block
        .trait_
        .as_ref()
        .is_some_and(|(_, trait_path, _)| {
            trait_path
                .segments
                .last()
                .is_some_and(|segment| segment.ident == "Default")
        })
}

#[cfg(test)]
mod tests {
    use super::is_default_impl;
    use crate::ImplBlock;

    #[test]
    fn test_usage() {
        let default_impl: syn::ItemImpl = syn::parse_quote! { impl Default for Foo {} };
        assert!(is_default_impl(ImplBlock(&default_impl)));
        let inherent: syn::ItemImpl = syn::parse_quote! { impl Foo {} };
        assert!(!is_default_impl(ImplBlock(&inherent)));
    }
}
