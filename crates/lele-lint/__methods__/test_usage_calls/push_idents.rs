use proc_macro2::TokenStream;
use proc_macro2::TokenTree;

use crate::rules;

pub fn push_idents(tokens: &TokenStream, idents: &mut Vec<String>) {
    for token in tokens.clone() {
        match token {
            TokenTree::Ident(ident) => idents.push(ident.to_string()),
            TokenTree::Group(group) => {
                rules::test_usage_shape::TestUsageCalls::push_idents(&group.stream(), idents);
            }
            TokenTree::Punct(_) | TokenTree::Literal(_) => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::push_idents;
    use crate::rules;
    use quote::quote;

    #[test]
    fn test_usage() {
        let tokens = quote!(a + b);
        let mut idents = Vec::new();
        push_idents(&tokens, &mut idents);
        assert_eq!(idents, vec!["a".to_string(), "b".to_string()]);
        let mut nested = Vec::new();
        rules::test_usage_shape::TestUsageCalls::push_idents(&quote!(c(d)), &mut nested);
        assert_eq!(nested, vec!["c".to_string(), "d".to_string()]);
    }
}
