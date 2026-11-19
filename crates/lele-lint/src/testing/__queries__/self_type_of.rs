use crate::methods;

pub(crate) fn self_type_of(impl_source: &str) -> Option<String> {
    let impl_block: syn::ItemImpl = syn::parse_str(impl_source).unwrap();
    methods::trait_impl_co_location::self_type_name(&impl_block)
}

#[cfg(test)]
mod tests {
    use super::self_type_of;

    #[test]
    fn test_usage() {
        assert_eq!(
            self_type_of("impl Tr for UnixTime {}"),
            Some("UnixTime".to_string())
        );
    }
}
