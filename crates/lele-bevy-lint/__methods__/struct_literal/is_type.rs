use crate::scan;

pub fn is_type(literal: &scan::StructLiteral, type_name: &str) -> bool {
    let last = literal
        .path
        .segments
        .last()
        .map(|segment| segment.ident.to_string());
    last.as_deref() == Some(type_name)
}

#[cfg(test)]
mod tests {
    use super::is_type;
    use crate::scan;

    #[test]
    fn test_usage() {
        let literal: syn::ExprStruct = syn::parse_str("scene::State { label, apply }").unwrap();
        assert!(is_type(&scan::StructLiteral(&literal), "State"));
        assert!(!is_type(&scan::StructLiteral(&literal), "Other"));
    }
}
