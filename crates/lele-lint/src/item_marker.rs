use atomic_delegate_macros::atomic_delegates;
use derive_more::Deref;
use syn::meta::ParseNestedMeta;

#[derive(Clone, Copy, Deref)]
pub struct ItemMarker<'a>(pub &'a syn::Attribute);

#[atomic_delegates]
impl ItemMarker<'_> {
    pub fn is_cfg_test(self) -> bool {}
    pub fn is_rustfmt_skip(self) -> bool {}
    pub fn derives_any_of(self, names: &[&str]) -> bool {}
}

#[rustfmt::skip]
impl ItemMarker<'_> {
    pub fn is_named(&self, name: &str) -> bool { self.path().segments.last().is_some_and(|segment| segment.ident == name) }
    pub fn is_atomic_delegates(&self) -> bool { self.is_named("atomic_delegates") }
    pub fn is_atomic_delegate(&self) -> bool { self.is_named("atomic_delegate") }
    pub fn is_test(&self) -> bool { self.is_named("test") }
    pub fn derives_data_shape(&self) -> bool { self.derives_any_of(&["Serialize", "Deserialize", "Parser", "Args", "Subcommand", "ValueEnum"]) }
    pub(crate) fn all_contains_test(meta: &ParseNestedMeta) -> syn::Result<()> {
        let mut found = false;
        let _ = meta.parse_nested_meta(|inner| {
            if inner.path.is_ident("test") {
                found = true;
            }
            Ok(())
        });
        if found {
            Ok(())
        } else {
            Err(meta.error("expected test"))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ItemMarker;

    #[test]
    fn test_usage() {
        let item: syn::ItemImpl = syn::parse_quote! {
            #[atomic_delegates]
            impl Foo { pub fn f(&self) {} }
        };
        let marker = item.attrs.first().map(ItemMarker).unwrap();
        assert!(marker.is_named("atomic_delegates"));
        assert!(!marker.is_named("other"));
        assert!(marker.is_atomic_delegates());
        assert!(!marker.is_atomic_delegate());
        assert!(!marker.is_test());

        let method: syn::ImplItemFn = syn::parse_quote! {
            #[atomic_delegate(Foo)]
            fn check(&self) {}
        };
        assert!(method
            .attrs
            .first()
            .map(ItemMarker)
            .unwrap()
            .is_atomic_delegate());

        let wire: syn::ItemStruct =
            syn::parse_quote! { #[derive(Parser)] struct Args { root: String } };
        assert!(wire
            .attrs
            .first()
            .map(ItemMarker)
            .unwrap()
            .derives_data_shape());
        let plain: syn::ItemStruct = syn::parse_quote! { #[derive(Clone)] struct X { value: u64 } };
        assert!(!plain
            .attrs
            .first()
            .map(ItemMarker)
            .unwrap()
            .derives_data_shape());
    }
}
