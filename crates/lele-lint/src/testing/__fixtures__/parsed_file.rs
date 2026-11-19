pub(crate) fn parsed_file(source: &str) -> syn::File {
    syn::parse_str(source).unwrap()
}

#[cfg(test)]
mod tests {
    use super::parsed_file;

    #[test]
    fn test_usage() {
        assert_eq!(parsed_file("pub struct A;").items.len(), 1);
    }
}
