pub fn single_argument_type(sig: &syn::Signature) -> Option<String> {
    let [syn::FnArg::Typed(argument)] = sig.inputs.iter().collect::<Vec<_>>()[..] else {
        return None;
    };
    let source = quote::ToTokens::to_token_stream(&*argument.ty).to_string();
    Some(source)
}

#[cfg(test)]
mod tests {
    use crate::rules;

    #[test]
    fn test_usage() {
        let one: syn::Signature = syn::parse_quote! { fn from_secs(secs: u64) -> Self };
        assert_eq!(
            rules::conversion_is_from::ConversionIsFrom::single_argument_type(&one).as_deref(),
            Some("u64")
        );
        let two: syn::Signature = syn::parse_quote! { fn pair(a: u8, b: u8) -> Self };
        assert!(rules::conversion_is_from::ConversionIsFrom::single_argument_type(&two).is_none());
        let none: syn::Signature = syn::parse_quote! { fn empty() -> Self };
        assert!(rules::conversion_is_from::ConversionIsFrom::single_argument_type(&none).is_none());
    }
}
