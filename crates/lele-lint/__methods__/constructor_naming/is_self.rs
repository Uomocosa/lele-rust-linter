pub fn is_self(ty: &syn::Type, self_name: &str) -> bool {
    matches!(
        ty,
        syn::Type::Path(path)
            if path.qself.is_none()
                && path.path.segments.len() == 1
                && path.path.segments.last().is_some_and(|segment| {
                    segment.arguments.is_none()
                        && (segment.ident == "Self" || segment.ident == self_name)
                })
    )
}

#[cfg(test)]
mod tests {
    use crate::rules;

    #[test]
    fn test_usage() {
        let self_type: syn::Type = syn::parse_quote! { Self };
        assert!(rules::constructor_naming::ConstructorNaming::is_self(
            &self_type, "Thing"
        ));
        let named: syn::Type = syn::parse_quote! { Thing };
        assert!(rules::constructor_naming::ConstructorNaming::is_self(
            &named, "Thing"
        ));
        let generic: syn::Type = syn::parse_quote! { Vec<Self> };
        assert!(!rules::constructor_naming::ConstructorNaming::is_self(
            &generic, "Thing"
        ));
    }
}
