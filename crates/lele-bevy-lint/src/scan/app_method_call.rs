use atomic_delegate_macros::atomic_delegates;
use derive_more::Deref;

#[derive(Clone, Copy, Deref)]
pub struct AppMethodCall<'a>(pub &'a syn::ExprMethodCall);

#[atomic_delegates]
impl AppMethodCall<'_> {
    pub fn system_args(&self) -> Option<Vec<String>> {}
    pub fn observer_args(&self) -> Option<Vec<String>> {}
}

#[cfg(test)]
mod tests {
    use crate::scan::AppMethodCall;

    #[test]
    fn test_usage() {
        let call: syn::ExprMethodCall = syn::parse_quote! { app.add_systems(Update, (tick, draw)) };
        assert_eq!(
            AppMethodCall(&call).system_args(),
            Some(vec!["tick".to_string(), "draw".to_string()])
        );
        assert_eq!(AppMethodCall(&call).observer_args(), None);
    }
}
