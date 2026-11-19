pub(crate) fn macro_of(source: &str) -> syn::Macro {
    let expr: syn::Expr = syn::parse_str(source).unwrap();
    let syn::Expr::Macro(found) = expr else {
        panic!("expected a macro");
    };
    found.mac
}

#[cfg(test)]
mod tests {
    use super::macro_of;

    #[test]
    fn test_usage() {
        assert_eq!(macro_of("vec![1, 2]").path.segments.len(), 1);
    }
}
