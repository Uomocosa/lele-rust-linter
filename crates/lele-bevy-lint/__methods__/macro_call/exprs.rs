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
    use crate::testing;

    #[test]
    fn test_usage() {
        assert_eq!(
            exprs(&scan::MacroCall(&testing::macro_of(
                "children![(Node, Text::new(\"x\")), Sprite]"
            )))
            .len(),
            2
        );
        assert_eq!(
            exprs(&scan::MacroCall(&testing::macro_of(
                "vec![State { label, apply }]"
            )))
            .len(),
            1
        );
        assert_eq!(
            exprs(&scan::MacroCall(&testing::macro_of("vec![0; 4]"))).len(),
            0
        );
    }
}
