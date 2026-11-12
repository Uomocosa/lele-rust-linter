use atomic_delegate_macros::atomic_delegates;
use derive_more::Deref;
use syn::spanned::Spanned;

const MAX_INLINE_STATEMENTS: usize = 3;

#[derive(Clone, Copy, Deref)]
pub struct FnBody<'a>(pub &'a syn::Block);

#[atomic_delegates]
impl FnBody<'_> {
    pub fn is_delegate_call(self) -> bool {}
    pub fn is_methods_dispatch(self) -> bool {}
    pub fn single_path_call_segments(self) -> Option<Vec<String>> {}
}

#[rustfmt::skip]
impl FnBody<'_> {
    pub fn is_short(&self) -> bool { self.stmts.len() <= MAX_INLINE_STATEMENTS }
    pub fn is_one_liner(&self) -> bool { self.stmts.len() <= 1 && self.stmts.first().is_none_or(|stmt| stmt.span().start().line == stmt.span().end().line) }
}

#[cfg(test)]
mod tests {
    use crate::FnBody;

    #[test]
    fn test_usage() {
        let empty: syn::Block = syn::parse_quote! {{}};
        assert!(FnBody(&empty).is_short());
        let three_stmts: syn::Block = syn::parse_quote! {{ let a = 1; let b = 2; a + b }};
        assert!(FnBody(&three_stmts).is_short());
        let four_stmts: syn::Block =
            syn::parse_quote! {{ let a = 1; let b = 2; let c = 3; a + b + c }};
        assert!(!FnBody(&four_stmts).is_short());
        let no_stmts: syn::Block = syn::parse_quote! {{}};
        assert!(FnBody(&no_stmts).is_one_liner());
        let one_line: syn::Block = syn::parse_quote! {{ let a = 1; }};
        assert!(FnBody(&one_line).is_one_liner());
        let multi_line: syn::Block =
            syn::parse_str("{\n match x {\n 0 => {}\n _ => {}\n }\n}").unwrap();
        assert!(!FnBody(&multi_line).is_one_liner());
        let two_stmts: syn::Block = syn::parse_quote! {{ let a = 1; let b = 2; }};
        assert!(!FnBody(&two_stmts).is_one_liner());
    }
}
