pub fn is_ok_or_some(call: &syn::ExprCall) -> bool {
    matches!(&*call.func, syn::Expr::Path(path)
        if path.path.segments.last().is_some_and(|segment| segment.ident == "Ok" || segment.ident == "Some"))
}

#[cfg(test)]
mod tests {
    use crate::rules;

    #[test]
    fn test_usage() {
        let ok: syn::Expr = syn::parse_quote! { Ok(1) };
        let some: syn::Expr = syn::parse_quote! { Some(1) };
        let other: syn::Expr = syn::parse_quote! { make(1) };
        let call = |expr: &syn::Expr| match expr {
            syn::Expr::Call(call) => call.clone(),
            _ => panic!("not a call"),
        };
        assert!(rules::no_trivial_accessors::NoTrivialAccessors::is_ok_or_some(&call(&ok)));
        assert!(rules::no_trivial_accessors::NoTrivialAccessors::is_ok_or_some(&call(&some)));
        assert!(!rules::no_trivial_accessors::NoTrivialAccessors::is_ok_or_some(&call(&other)));
    }
}
