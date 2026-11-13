use crate::ImplMethod;

pub fn has_any_real_constructor(impl_block: &syn::ItemImpl) -> bool {
    impl_block.items.iter().any(
        |item| matches!(item, syn::ImplItem::Fn(method) if ImplMethod(method).is_real_constructor()),
    )
}

#[cfg(test)]
mod tests {
    use crate::rules;

    #[test]
    fn test_usage() {
        let with_ctor: syn::ItemImpl = syn::parse_quote! { impl Thing { pub fn parse(raw: &str) -> Result<Self, Error> { Err(Error::Bad) } } };
        assert!(
            rules::constructor_no_skip::ConstructorNoSkip::has_any_real_constructor(&with_ctor)
        );
        let without_ctor: syn::ItemImpl =
            syn::parse_quote! { impl Thing { pub fn get(&self) -> u8 { self.a } } };
        assert!(
            !rules::constructor_no_skip::ConstructorNoSkip::has_any_real_constructor(&without_ctor)
        );
    }
}
