use crate::ImplBlock;

pub fn trait_ident(impl_block: ImplBlock) -> Option<String> {
    impl_block
        .trait_
        .as_ref()
        .and_then(|(_, path, _)| path.segments.last())
        .map(|segment| segment.ident.to_string())
}

#[cfg(test)]
mod tests {
    use super::trait_ident;
    use crate::ImplBlock;

    #[test]
    fn test_usage() {
        let rule: syn::ItemImpl = syn::parse_quote! { impl Rule for Foo {} };
        assert_eq!(trait_ident(ImplBlock(&rule)).as_deref(), Some("Rule"));
        let inherent: syn::ItemImpl = syn::parse_quote! { impl Foo {} };
        assert_eq!(trait_ident(ImplBlock(&inherent)), None);
    }
}
