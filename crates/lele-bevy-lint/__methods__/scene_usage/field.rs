pub fn field<'a>(literal: &'a syn::ExprStruct, name: &str) -> Option<&'a syn::Expr> {
    literal
        .fields
        .iter()
        .find(|value| matches!(&value.member, syn::Member::Named(ident) if ident == name))
        .map(|value| &value.expr)
}

#[cfg(test)]
mod tests {
    use super::field;

    #[test]
    fn test_usage() {
        let literal: syn::ExprStruct = syn::parse_quote! { Scene { kind: Png, build } };
        assert!(field(&literal, "kind").is_some());
        assert!(field(&literal, "build").is_some());
        assert!(field(&literal, "states").is_none());
    }
}
