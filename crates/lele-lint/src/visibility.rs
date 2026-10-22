use derive_more::Deref;

#[derive(Clone, Copy, Deref)]
pub struct Visibility<'a>(pub &'a syn::Visibility);

#[rustfmt::skip]
impl Visibility<'_> {
    pub fn is_exposed(&self) -> bool {
        match **self {
            syn::Visibility::Public(_) => true,
            syn::Visibility::Restricted(r) => r.path.segments.len() == 1 && r.path.segments.first().is_some_and(|s| s.ident == "crate"),
            syn::Visibility::Inherited => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Visibility;

    #[test]
    fn test_usage() {
        let public: syn::Visibility = syn::parse_quote! { pub };
        assert!(Visibility(&public).is_exposed());
        let crate_vis: syn::Visibility = syn::parse_quote! { pub(crate) };
        assert!(Visibility(&crate_vis).is_exposed());
        let super_vis: syn::Visibility = syn::parse_quote! { pub(super) };
        assert!(!Visibility(&super_vis).is_exposed());
        assert!(!Visibility(&syn::Visibility::Inherited).is_exposed());
    }
}
