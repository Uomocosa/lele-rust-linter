use syn::punctuated::Punctuated;

#[must_use]
pub fn macro_exprs(mac: &syn::Macro) -> Vec<syn::Expr> {
    mac.parse_body_with(Punctuated::<syn::Expr, syn::Token![,]>::parse_terminated)
        .map(|exprs| exprs.into_iter().collect())
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::macro_exprs;

    fn mac(source: &str) -> syn::Macro {
        let expr: syn::Expr = syn::parse_str(source).unwrap();
        let syn::Expr::Macro(found) = expr else {
            panic!("expected a macro");
        };
        found.mac
    }

    #[test]
    fn test_usage() {
        assert_eq!(
            macro_exprs(&mac("children![(Node, Text::new(\"x\")), Sprite]")).len(),
            2
        );
        assert_eq!(macro_exprs(&mac("vec![State { label, apply }]")).len(), 1);
        assert_eq!(macro_exprs(&mac("vec![0; 4]")).len(), 0);
    }
}
