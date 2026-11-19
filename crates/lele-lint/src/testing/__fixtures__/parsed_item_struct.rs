pub(crate) fn parsed_item_struct(code: &str) -> syn::ItemStruct {
    syn::parse_str(code).unwrap()
}

#[cfg(test)]
mod tests {
    use super::parsed_item_struct;

    #[test]
    fn test_usage() {
        assert_eq!(parsed_item_struct("pub struct X(pub u64);").ident, "X");
    }
}
