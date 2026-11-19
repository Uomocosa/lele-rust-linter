use crate::methods;

pub(crate) fn impl_trait_path_of(impl_source: &str) -> Option<Vec<String>> {
    let impl_block: syn::ItemImpl = syn::parse_str(impl_source).unwrap();
    methods::trait_impl_co_location::impl_trait_path(&impl_block)
}

#[cfg(test)]
mod tests {
    use super::impl_trait_path_of;

    #[test]
    fn test_usage() {
        assert_eq!(
            impl_trait_path_of("impl Clock for UnixTime {}"),
            Some(vec!["Clock".to_string()])
        );
    }
}
