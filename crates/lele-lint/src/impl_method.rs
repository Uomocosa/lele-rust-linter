use derive_more::Deref;

use quote::ToTokens;

use crate::FnBody;
use crate::ImplItemGroup;

#[derive(Clone, Copy, Deref)]
pub struct ImplMethod<'a>(pub &'a syn::ImplItemFn);

#[rustfmt::skip]
impl ImplMethod<'_> {
    pub fn is_real_constructor(&self) -> bool {
        self.sig.receiver().is_none()
            && self.sig.output.to_token_stream().to_string().split_whitespace().any(|token| token == "Self")
            && !FnBody(&self.block).is_delegate_call()
            && !FnBody(&self.block).is_methods_dispatch()
    }
    pub(crate) fn order_group(self) -> ImplItemGroup {
        let returns_self = self.sig.output.to_token_stream().to_string().split_whitespace().any(|token| token == "Self");
        if self.sig.receiver().is_none() && returns_self {
            ImplItemGroup::Constructor
        } else if matches!(self.vis, syn::Visibility::Public(_)) {
            ImplItemGroup::PubFn
        } else {
            ImplItemGroup::CrateFn
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ImplMethod;

    #[test]
    fn test_usage() {
        let real: syn::ImplItemFn =
            syn::parse_quote! { pub fn new() -> Self { Self { a: 1, b: 2 } } };
        assert!(ImplMethod(&real).is_real_constructor());
        let delegate: syn::ImplItemFn =
            syn::parse_quote! { pub fn new() -> Self { foo_new::new() } };
        assert!(!ImplMethod(&delegate).is_real_constructor());
        let dispatch: syn::ImplItemFn =
            syn::parse_quote! { pub fn load() -> Self { crate::methods::foo::load() } };
        assert!(!ImplMethod(&dispatch).is_real_constructor());
        let method: syn::ImplItemFn = syn::parse_quote! { pub fn get(&self) -> u8 { self.a } };
        assert!(!ImplMethod(&method).is_real_constructor());
        let unit: syn::ImplItemFn = syn::parse_quote! { pub fn reset() { let _ = 1; } };
        assert!(!ImplMethod(&unit).is_real_constructor());
        let fallible: syn::ImplItemFn = syn::parse_quote! { pub fn parse(raw: &str) -> Result<Self, Error> { Err(Error::Bad) } };
        assert!(ImplMethod(&fallible).is_real_constructor());
        let helper: syn::ImplItemFn =
            syn::parse_quote! { pub(crate) fn base(hits: u32) -> u32 { hits.saturating_mul(10) } };
        assert!(!ImplMethod(&helper).is_real_constructor());

        let shell_ctor: syn::ImplItemFn =
            syn::parse_quote! { pub fn from_save(slot: u8) -> Self {} };
        assert_eq!(
            ImplMethod(&shell_ctor).order_group(),
            crate::ImplItemGroup::Constructor
        );
        let pub_shell: syn::ImplItemFn = syn::parse_quote! { pub fn run(&self) {} };
        assert_eq!(
            ImplMethod(&pub_shell).order_group(),
            crate::ImplItemGroup::PubFn
        );
        let crate_shell: syn::ImplItemFn = syn::parse_quote! { pub(crate) fn step(&self) {} };
        assert_eq!(
            ImplMethod(&crate_shell).order_group(),
            crate::ImplItemGroup::CrateFn
        );
    }
}
