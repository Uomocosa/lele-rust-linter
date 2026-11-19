pub(crate) fn method_call(source: &str) -> syn::ExprMethodCall {
    let expr: syn::Expr = syn::parse_str(source).unwrap();
    let syn::Expr::MethodCall(call) = expr else {
        panic!("expected a method call");
    };
    call
}

#[cfg(test)]
mod tests {
    use super::method_call;

    #[test]
    fn test_usage() {
        assert_eq!(
            method_call("app.add_systems(Update, tick)").method,
            "add_systems"
        );
    }
}
