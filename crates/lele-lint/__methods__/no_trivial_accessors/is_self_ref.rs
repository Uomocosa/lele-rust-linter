pub fn is_self_ref(expr: &syn::Expr) -> bool {
    if let syn::Expr::Path(path) = expr {
        return path.path.segments.last().is_some_and(|s| s.ident == "self");
    }
    false
}

// no test_usage necessary
