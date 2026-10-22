use atomic_delegate_macros::atomic_delegates;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum HarnessCall {
    Missing,
    Unchecked,
    Checked,
}

#[atomic_delegates]
impl HarnessCall {
    pub fn detect(body: &syn::Block, is_run_imported: bool) -> Self {}
}

#[cfg(test)]
mod tests {
    use crate::scan::HarnessCall;

    #[test]
    fn test_usage() {
        let body: syn::Block = syn::parse_quote! {{ let _ = 1; }};
        assert_eq!(HarnessCall::detect(&body, true), HarnessCall::Missing);
    }
}
