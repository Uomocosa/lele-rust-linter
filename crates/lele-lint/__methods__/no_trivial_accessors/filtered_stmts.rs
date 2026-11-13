use crate::rules;

pub fn filtered_stmts(block: &syn::Block) -> Vec<&syn::Stmt> {
    block
        .stmts
        .iter()
        .filter(|s| !rules::no_trivial_accessors::NoTrivialAccessors::is_log_macro(s))
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::rules;

    #[test]
    fn test_usage() {
        let block: syn::Block = syn::parse_quote! { { let a = 1; a } };
        assert_eq!(
            rules::no_trivial_accessors::NoTrivialAccessors::filtered_stmts(&block).len(),
            2
        );
    }
}
