use atomic_delegate_macros::atomic_delegates;
use derive_more::Deref;

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
    }
}
