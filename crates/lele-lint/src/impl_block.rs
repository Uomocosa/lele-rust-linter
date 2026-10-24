use atomic_delegate_macros::atomic_delegates;
use derive_more::Deref;

#[derive(Clone, Copy, Deref)]
pub struct ImplBlock<'a>(pub &'a syn::ItemImpl);

#[atomic_delegates]
impl ImplBlock<'_> {
    pub fn delegate_body_name(self, method: &syn::Ident) -> String {}
}

#[rustfmt::skip]
impl ImplBlock<'_> {
    pub fn is_default_impl(&self) -> bool {
        self.trait_.as_ref().is_some_and(|(_, trait_path, _)| trait_path.segments.last().is_some_and(|s| s.ident == "Default"))
    }
    pub fn self_type_last(&self) -> Option<String> {
        match &*self.self_ty {
            syn::Type::Path(tp) => tp.path.segments.last().map(|s| s.ident.to_string()),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ImplBlock;

    #[test]
    fn test_usage() {
        let default_impl: syn::ItemImpl =
            syn::parse_quote! { impl Default for Foo { fn default() -> Self { Self } } };
        assert!(ImplBlock(&default_impl).is_default_impl());
        assert_eq!(
            ImplBlock(&default_impl).self_type_last().as_deref(),
            Some("Foo")
        );
        let inherent: syn::ItemImpl = syn::parse_quote! { impl crate::a::Bar { fn f(&self) {} } };
        assert!(!ImplBlock(&inherent).is_default_impl());
        assert_eq!(
            ImplBlock(&inherent).self_type_last().as_deref(),
            Some("Bar")
        );
        let reference: syn::ItemImpl = syn::parse_quote! { impl Trait for &Bar {} };
        assert_eq!(ImplBlock(&reference).self_type_last(), None);
    }
}
