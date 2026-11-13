pub fn takes_self_typed_argument(sig: &syn::Signature, self_name: &str) -> bool {
    sig.inputs.iter().any(|input| {
        let syn::FnArg::Typed(typed) = input else {
            return false;
        };
        quote::ToTokens::to_token_stream(&*typed.ty)
            .into_iter()
            .any(|token| matches!(token, proc_macro2::TokenTree::Ident(ident) if ident == "Self" || ident == self_name))
    })
}

#[cfg(test)]
mod tests {
    use crate::rules;

    #[test]
    fn test_usage() {
        let merge: syn::Signature = syn::parse_quote! { fn merge(&self, other: Self) -> Self };
        assert!(
            rules::constructor_naming::ConstructorNaming::takes_self_typed_argument(
                &merge, "Thing"
            )
        );
        let named: syn::Signature = syn::parse_quote! { fn wrap(thing: Thing) -> Thing };
        assert!(
            rules::constructor_naming::ConstructorNaming::takes_self_typed_argument(
                &named, "Thing"
            )
        );
        let plain: syn::Signature = syn::parse_quote! { fn build(name: String) -> Self };
        assert!(
            !rules::constructor_naming::ConstructorNaming::takes_self_typed_argument(
                &plain, "Thing"
            )
        );
    }
}
