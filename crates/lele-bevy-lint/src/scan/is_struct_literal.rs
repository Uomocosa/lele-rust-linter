#[must_use]
pub fn is_struct_literal(node: &syn::ExprStruct, type_name: &str, field: &str) -> bool {
    let is_type = node
        .path
        .segments
        .last()
        .is_some_and(|segment| segment.ident == type_name);
    is_type
        && node
            .fields
            .iter()
            .any(|value| matches!(&value.member, syn::Member::Named(name) if name == field))
}

#[cfg(test)]
mod tests {
    use super::is_struct_literal;

    fn literal(source: &str) -> syn::ExprStruct {
        syn::parse_str(source).unwrap()
    }

    #[test]
    fn test_usage() {
        assert!(is_struct_literal(
            &literal("scene::State { label, apply }"),
            "State",
            "apply"
        ));
        assert!(!is_struct_literal(
            &literal("State { label }"),
            "State",
            "apply"
        ));
        assert!(!is_struct_literal(
            &literal("Other { apply }"),
            "State",
            "apply"
        ));
    }
}
