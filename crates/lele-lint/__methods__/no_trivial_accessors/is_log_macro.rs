pub fn is_log_macro(stmt: &syn::Stmt) -> bool {
    let mac = match stmt {
        syn::Stmt::Macro(m) => &m.mac,
        syn::Stmt::Expr(syn::Expr::Macro(em), _) => &em.mac,
        _ => return false,
    };
    let segments: Vec<String> = mac
        .path
        .segments
        .iter()
        .map(|s| s.ident.to_string())
        .collect();
    let path = segments.join("::");
    if path == "println" || path == "eprintln" {
        return true;
    }
    if path.ends_with("::println") || path.ends_with("::eprintln") {
        return true;
    }
    let last = segments.last().cloned().unwrap_or_default();
    let is_log_ident = matches!(last.as_str(), "info" | "debug" | "warn" | "error" | "trace");
    if !is_log_ident {
        return false;
    }
    segments
        .iter()
        .any(|s| s == "tracing" || s == "log" || s == "tracing_subscriber")
}

// no test_usage necessary
