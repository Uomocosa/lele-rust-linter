use quote::ToTokens;

use crate::FnBody;
use crate::ImplMethod;

pub fn is_real_constructor(impl_method: ImplMethod) -> bool {
    impl_method.sig.receiver().is_none()
        && impl_method
            .sig
            .output
            .to_token_stream()
            .to_string()
            .split_whitespace()
            .any(|token| token == "Self")
        && !FnBody(&impl_method.block).is_delegate_call()
        && !FnBody(&impl_method.block).is_methods_dispatch()
}

#[cfg(test)]
mod tests {
    use super::is_real_constructor;
    use crate::ImplMethod;

    #[test]
    fn test_usage() {
        let real: syn::ImplItemFn = syn::parse_quote! { pub fn new() -> Self { Self { a: 1 } } };
        assert!(is_real_constructor(ImplMethod(&real)));
        let method: syn::ImplItemFn = syn::parse_quote! { pub fn get(&self) -> u8 { self.a } };
        assert!(!is_real_constructor(ImplMethod(&method)));
    }
}
