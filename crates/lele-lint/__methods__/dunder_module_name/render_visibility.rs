pub fn render_visibility(vis: &syn::Visibility) -> String {
    match vis {
        syn::Visibility::Public(_) => "pub".to_string(),
        syn::Visibility::Inherited => String::new(),
        syn::Visibility::Restricted(restricted) => {
            let segments: Vec<String> = restricted
                .path
                .segments
                .iter()
                .map(|segment| segment.ident.to_string())
                .collect();
            if segments.len() == 1 && segments.first().is_some_and(|first| first == "crate") {
                "pub(crate)".to_string()
            } else {
                format!("pub(in {})", segments.join("::"))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::rules;

    #[test]
    fn test_usage() {
        let public: syn::Visibility = syn::parse_quote! { pub };
        assert_eq!(
            rules::dunder_module_name::DunderModuleName::render_visibility(&public),
            "pub"
        );
        let crate_only: syn::Visibility = syn::parse_quote! { pub(crate) };
        assert_eq!(
            rules::dunder_module_name::DunderModuleName::render_visibility(&crate_only),
            "pub(crate)"
        );
        assert_eq!(
            rules::dunder_module_name::DunderModuleName::render_visibility(
                &syn::Visibility::Inherited
            ),
            ""
        );
    }
}
