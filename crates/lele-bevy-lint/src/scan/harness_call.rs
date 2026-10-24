use atomic_delegate_macros::atomic_delegates;

use crate::scan::basic::constants::HARNESS_RUN;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum HarnessCall {
    Missing,
    Unchecked,
    Checked,
}

#[atomic_delegates]
impl HarnessCall {
    pub fn from_fn_body(body: &syn::Block, is_run_imported: bool) -> Self {}
    pub(crate) fn is_harness_path(func: &syn::Expr, is_run_imported: bool) -> bool {}
}

#[rustfmt::skip]
impl HarnessCall {
    pub(crate) fn is_harness_run(expr: &syn::Expr, is_run_imported: bool) -> bool {
        match expr {
            syn::Expr::Paren(inner) => Self::is_harness_run(&inner.expr, is_run_imported),
            syn::Expr::Group(inner) => Self::is_harness_run(&inner.expr, is_run_imported),
            syn::Expr::Call(call) => Self::is_harness_path(&call.func, is_run_imported),
            syn::Expr::MethodCall(call) => call.method == HARNESS_RUN,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::scan::HarnessCall;

    #[test]
    fn test_usage() {
        let body: syn::Block = syn::parse_quote! {{ let _ = 1; }};
        assert_eq!(HarnessCall::from_fn_body(&body, true), HarnessCall::Missing);
    }
}
