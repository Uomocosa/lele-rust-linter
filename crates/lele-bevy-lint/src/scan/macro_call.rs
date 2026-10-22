use atomic_delegate_macros::atomic_delegates;
use derive_more::Deref;

#[derive(Clone, Copy, Deref)]
pub struct MacroCall<'a>(pub &'a syn::Macro);

#[atomic_delegates]
impl MacroCall<'_> {
    pub fn exprs(&self) -> Vec<syn::Expr> {}
}

#[cfg(test)]
mod tests {
    use super::MacroCall;

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
            MacroCall(&mac("children![(Node, Text::new(\"x\")), Sprite]"))
                .exprs()
                .len(),
            2
        );
        assert_eq!(
            MacroCall(&mac("vec![State { label, apply }]"))
                .exprs()
                .len(),
            1
        );
        assert_eq!(MacroCall(&mac("vec![0; 4]")).exprs().len(), 0);
    }
}
