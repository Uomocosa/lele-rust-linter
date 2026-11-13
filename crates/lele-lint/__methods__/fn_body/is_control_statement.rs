use crate::FnBody;

pub fn is_control_statement(block: FnBody) -> bool {
    let [syn::Stmt::Expr(expr, _)] = block.stmts.as_slice() else {
        return false;
    };
    matches!(
        expr,
        syn::Expr::Match(_)
            | syn::Expr::If(_)
            | syn::Expr::ForLoop(_)
            | syn::Expr::While(_)
            | syn::Expr::Loop(_)
            | syn::Expr::Block(_)
    )
}

#[cfg(test)]
mod tests {
    use crate::FnBody;

    #[test]
    fn test_usage() {
        let matched: syn::Block = syn::parse_quote! {{ match x { _ => 1 } }};
        assert!(FnBody(&matched).is_control_statement());
        let call: syn::Block = syn::parse_quote! {{ foo::run(self) }};
        assert!(!FnBody(&call).is_control_statement());
        let two: syn::Block = syn::parse_quote! {{ let a = 1; if a > 0 { a } else { 0 } }};
        assert!(!FnBody(&two).is_control_statement());
    }
}
