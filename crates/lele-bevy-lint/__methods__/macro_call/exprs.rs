use syn::punctuated::Punctuated;

use crate::scan;

pub fn exprs(mac: &scan::MacroCall) -> Vec<syn::Expr> {
    let parsed = mac.parse_body_with(Punctuated::<syn::Expr, syn::Token![,]>::parse_terminated);
    parsed
        .map(|exprs| exprs.into_iter().collect())
        .unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::exprs;
    use crate::scan;

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
            exprs(&scan::MacroCall(&mac(
                "children![(Node, Text::new(\"x\")), Sprite]"
            )))
            .len(),
            2
        );
        assert_eq!(
            exprs(&scan::MacroCall(&mac("vec![State { label, apply }]"))).len(),
            1
        );
        assert_eq!(exprs(&scan::MacroCall(&mac("vec![0; 4]"))).len(), 0);
    }
}
