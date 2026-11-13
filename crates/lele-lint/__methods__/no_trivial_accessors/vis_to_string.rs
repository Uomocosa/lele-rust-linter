pub fn vis_to_string(vis: &syn::Visibility) -> Option<String> {
    match vis {
        syn::Visibility::Public(_) => Some("pub".to_string()),
        syn::Visibility::Restricted(res) => {
            let s = quote::quote!(#res).to_string();
            Some(s.replace(" )", ")").replace("( ", "("))
        }
        syn::Visibility::Inherited => None,
    }
}

#[cfg(test)]
mod tests {
    use crate::rules;

    #[test]
    fn test_usage() {
        let public: syn::Visibility = syn::parse_quote! { pub };
        assert_eq!(
            rules::no_trivial_accessors::NoTrivialAccessors::vis_to_string(&public).as_deref(),
            Some("pub")
        );
        let restricted: syn::Visibility = syn::parse_quote! { pub(crate) };
        assert!(
            rules::no_trivial_accessors::NoTrivialAccessors::vis_to_string(&restricted)
                .is_some_and(|text| text.contains("crate"))
        );
        assert!(
            rules::no_trivial_accessors::NoTrivialAccessors::vis_to_string(
                &syn::Visibility::Inherited
            )
            .is_none()
        );
    }
}
