pub fn has_fn(impl_block: &syn::ItemImpl) -> bool {
    impl_block
        .items
        .iter()
        .any(|item| matches!(item, syn::ImplItem::Fn(_)))
}

#[cfg(test)]
mod tests {
    use crate::rules;

    #[test]
    fn test_usage() {
        let with_fn: syn::ItemImpl = syn::parse_quote! { impl Thing { fn run(&self) {} } };
        assert!(rules::delegate_macro::DelegateMacro::has_fn(&with_fn));
        let without_fn: syn::ItemImpl = syn::parse_quote! { impl Thing { const N: u8 = 1; } };
        assert!(!rules::delegate_macro::DelegateMacro::has_fn(&without_fn));
    }
}
