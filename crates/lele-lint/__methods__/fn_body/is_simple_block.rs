use crate::FnBody;

pub fn is_simple_block(block: FnBody) -> bool {
    let [syn::Stmt::Expr(expr, _)] = block.stmts.as_slice() else {
        return false;
    };
    let plain = |body: &syn::Expr| match body {
        syn::Expr::Block(inner) => FnBody(&inner.block).is_plain_body(),
        other => FnBody(&syn::Block {
            brace_token: syn::token::Brace::default(),
            stmts: vec![syn::Stmt::Expr(other.clone(), None)],
        })
        .is_plain_body(),
    };
    match expr {
        syn::Expr::Match(matched) => matched.arms.iter().all(|arm| plain(&arm.body)),
        syn::Expr::If(first) => {
            let mut current = first;
            loop {
                if !FnBody(&current.then_branch).is_plain_body() {
                    return false;
                }
                let Some((_, otherwise)) = &current.else_branch else {
                    return true;
                };
                match otherwise.as_ref() {
                    syn::Expr::If(next) => current = next,
                    other => return plain(other),
                }
            }
        }
        syn::Expr::ForLoop(looped) => FnBody(&looped.body).is_plain_body(),
        syn::Expr::While(looped) => FnBody(&looped.body).is_plain_body(),
        syn::Expr::Loop(looped) => FnBody(&looped.body).is_plain_body(),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use crate::FnBody;

    #[test]
    fn test_usage() {
        let plain_match: syn::Block = syn::parse_quote! {{ match x { 0 => 1, _ => 2 } }};
        assert!(FnBody(&plain_match).is_simple_block());
        let two_in_arm: syn::Block =
            syn::parse_quote! {{ match x { 0 => { let a = 1; a } _ => 2 } }};
        assert!(!FnBody(&two_in_arm).is_simple_block());
        let nested_if: syn::Block =
            syn::parse_quote! {{ match x { 0 => if y { 1 } else { 2 }, _ => 3 } }};
        assert!(!FnBody(&nested_if).is_simple_block());
        let chain: syn::Block = syn::parse_quote! {{ if a { 1 } else if b { 2 } else { 3 } }};
        assert!(FnBody(&chain).is_simple_block());
        let loop_with_if: syn::Block = syn::parse_quote! {{ for i in v { if i { g() } } }};
        assert!(!FnBody(&loop_with_if).is_simple_block());
        let plain_for: syn::Block = syn::parse_quote! {{ for i in v { g(i) } }};
        assert!(FnBody(&plain_for).is_simple_block());
        let call: syn::Block = syn::parse_quote! {{ foo::run(self) }};
        assert!(!FnBody(&call).is_simple_block());
    }
}
