pub fn has_generic_argument(sig: &syn::Signature) -> bool {
    let [syn::FnArg::Typed(argument)] = sig.inputs.iter().collect::<Vec<_>>()[..] else {
        return false;
    };
    match &*argument.ty {
        syn::Type::ImplTrait(_) => true,
        syn::Type::Path(path) => path.path.get_ident().is_some_and(|ident| {
            sig.generics
                .type_params()
                .any(|parameter| parameter.ident == *ident)
        }),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use crate::rules;

    #[test]
    fn test_usage() {
        let plain: syn::Signature = syn::parse_quote! { fn sized(size: usize) -> Self };
        assert!(!rules::conversion_is_from::ConversionIsFrom::has_generic_argument(&plain));
        let impl_trait: syn::Signature =
            syn::parse_quote! { fn named(name: impl Into<String>) -> Self };
        assert!(rules::conversion_is_from::ConversionIsFrom::has_generic_argument(&impl_trait));
        let type_parameter: syn::Signature =
            syn::parse_quote! { fn tagged<T: Into<String>>(tag: T) -> Self };
        assert!(rules::conversion_is_from::ConversionIsFrom::has_generic_argument(&type_parameter));
        let concrete: syn::Signature = syn::parse_quote! { fn wrap<T>(tag: String) -> Self };
        assert!(!rules::conversion_is_from::ConversionIsFrom::has_generic_argument(&concrete));
    }
}
