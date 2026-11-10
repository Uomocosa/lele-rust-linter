use crate::scan;

pub fn has_field(literal: &scan::StructLiteral, field: &str) -> bool {
    let mut named = literal
        .fields
        .iter()
        .filter_map(|value| match &value.member {
            syn::Member::Named(name) => Some(name.to_string()),
            _ => None,
        });
    named.any(|name| name == field)
}

#[cfg(test)]
mod tests {
    use super::has_field;
    use crate::scan;

    #[test]
    fn test_usage() {
        let literal: syn::ExprStruct = syn::parse_str("scene::State { label, apply }").unwrap();
        assert!(has_field(&scan::StructLiteral(&literal), "apply"));
        assert!(!has_field(&scan::StructLiteral(&literal), "other"));
    }
}
