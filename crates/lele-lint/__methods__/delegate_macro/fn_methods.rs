pub fn fn_methods(impl_block: &syn::ItemImpl) -> Vec<&syn::ImplItemFn> {
    impl_block
        .items
        .iter()
        .filter_map(|item| match item {
            syn::ImplItem::Fn(method) => Some(method),
            _ => None,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::rules;

    #[test]
    fn test_usage() {
        let block: syn::ItemImpl = syn::parse_quote! { impl Thing { const N: u8 = 1; fn run(&self) {} fn stop(&self) {} } };
        let methods = rules::delegate_macro::DelegateMacro::fn_methods(&block);
        assert_eq!(methods.len(), 2);
        assert_eq!(methods[0].sig.ident, "run");
        assert_eq!(methods[1].sig.ident, "stop");
    }
}
