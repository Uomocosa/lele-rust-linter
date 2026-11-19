use proc_macro2::TokenStream;

use crate::rules;

pub fn idents_in(tokens: &TokenStream) -> Vec<String> {
    let mut idents = Vec::new();
    rules::test_usage_shape::TestUsageCalls::push_idents(tokens, &mut idents);
    idents
}

#[cfg(test)]
mod tests {
    use super::idents_in;
    use crate::rules;
    use quote::quote;

    #[test]
    fn test_usage() {
        let tokens = quote!(foo::bar(1, "two"));
        let idents = idents_in(&tokens);
        assert_eq!(idents, vec!["foo".to_string(), "bar".to_string()]);
        assert_eq!(
            rules::test_usage_shape::TestUsageCalls::idents_in(&tokens),
            idents
        );
    }
}
