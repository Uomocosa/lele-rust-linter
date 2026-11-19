use crate::methods;

pub(crate) fn impl_type_path_of(impl_source: &str) -> Option<Vec<String>> {
    let impl_block: syn::ItemImpl = syn::parse_str(impl_source).unwrap();
    methods::trait_impl_co_location::impl_type_path(&impl_block)
}

#[cfg(test)]
mod tests {
    use super::impl_type_path_of;

    #[test]
    fn test_usage() {
        assert_eq!(
            impl_type_path_of("impl Clock for UnixTime {}"),
            Some(vec!["UnixTime".to_string()])
        );
    }
}
