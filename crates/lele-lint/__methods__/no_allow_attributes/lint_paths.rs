pub fn lint_paths(attr: &syn::Attribute) -> Vec<String> {
    attr.parse_args_with(syn::punctuated::Punctuated::<syn::Path, syn::Token![,]>::parse_terminated)
        .map(|paths| {
            paths
                .iter()
                .map(|path| {
                    path.segments
                        .iter()
                        .map(|segment| segment.ident.to_string())
                        .collect::<Vec<_>>()
                        .join("::")
                })
                .collect()
        })
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use crate::rules;

    #[test]
    fn test_usage() {
        let item: syn::ItemFn = syn::parse_quote! {
            #[allow(clippy::unwrap_used, dead_code)]
            fn f() {}
        };
        assert_eq!(
            rules::no_allow_attributes::NoAllowAttributes::lint_paths(&item.attrs[0]),
            ["clippy::unwrap_used", "dead_code"]
        );
    }
}
