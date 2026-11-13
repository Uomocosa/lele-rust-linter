use crate::rules;

pub fn mentions_name(tokens: proc_macro2::TokenStream, name: &str) -> bool {
    tokens.into_iter().any(|token| match token {
        proc_macro2::TokenTree::Ident(ident) => ident == name,
        proc_macro2::TokenTree::Literal(literal) => {
            let text = literal.to_string();
            text.contains(&format!("{{{name}}}")) || text.contains(&format!("{{{name}:"))
        }
        proc_macro2::TokenTree::Group(group) => {
            rules::no_stringified_errors::NoStringifiedErrors::mentions_name(group.stream(), name)
        }
        proc_macro2::TokenTree::Punct(_) => false,
    })
}

#[cfg(test)]
mod tests {
    use crate::rules;

    #[test]
    fn test_usage() {
        let ident = quote::quote! { format_error(err) };
        assert!(rules::no_stringified_errors::NoStringifiedErrors::mentions_name(ident, "err"));
        let literal = quote::quote! { format!("failed: {err}") };
        assert!(rules::no_stringified_errors::NoStringifiedErrors::mentions_name(literal, "err"));
        let unrelated = quote::quote! { format!("failed: {other}") };
        assert!(
            !rules::no_stringified_errors::NoStringifiedErrors::mentions_name(unrelated, "err")
        );
    }
}
