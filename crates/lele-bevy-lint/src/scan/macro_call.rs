use atomic_delegate_macros::atomic_delegates;
use derive_more::Deref;

#[derive(Clone, Deref)]
pub struct MacroCall<'a>(pub &'a syn::Macro);

#[atomic_delegates]
impl MacroCall<'_> {
    pub fn exprs(&self) -> Vec<syn::Expr> {}
}

#[cfg(test)]
mod tests {
    use super::MacroCall;
    use crate::testing;

    #[test]
    fn test_usage() {
        assert_eq!(
            MacroCall(&testing::macro_of(
                "children![(Node, Text::new(\"x\")), Sprite]"
            ))
            .exprs()
            .len(),
            2
        );
        assert_eq!(
            MacroCall(&testing::macro_of("vec![State { label, apply }]"))
                .exprs()
                .len(),
            1
        );
        assert_eq!(MacroCall(&testing::macro_of("vec![0; 4]")).exprs().len(), 0);
    }
}
