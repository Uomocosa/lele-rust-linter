pub fn self_type_name(item: &syn::ItemImpl) -> Option<String> {
    let mut self_type = &*item.self_ty;
    loop {
        match self_type {
            syn::Type::Path(type_path) => {
                return type_path
                    .path
                    .segments
                    .last()
                    .map(|segment| segment.ident.to_string());
            }
            syn::Type::Reference(reference) => self_type = &reference.elem,
            syn::Type::Paren(paren) => self_type = &paren.elem,
            _ => return None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::self_type_name;

    fn self_type_of(impl_source: &str) -> Option<String> {
        let impl_block: syn::ItemImpl = syn::parse_str(impl_source).unwrap();
        self_type_name(&impl_block)
    }

    #[test]
    fn test_usage() {
        assert_eq!(
            self_type_of("impl Tr for UnixTime {}"),
            Some("UnixTime".to_string())
        );
        assert_eq!(
            self_type_of("impl<T> Tr for Wrapper<T> {}"),
            Some("Wrapper".to_string())
        );
    }

    #[test]
    fn test_usage_unwraps_references_and_parens() {
        assert_eq!(
            self_type_of("impl Tr for &UnixTime {}"),
            Some("UnixTime".to_string())
        );
        assert_eq!(
            self_type_of("impl Tr for &mut UnixTime {}"),
            Some("UnixTime".to_string())
        );
        assert_eq!(
            self_type_of("impl Tr for (UnixTime) {}"),
            Some("UnixTime".to_string())
        );
        assert_eq!(
            self_type_of("impl Tr for &&(UnixTime) {}"),
            Some("UnixTime".to_string())
        );
    }

    #[test]
    fn test_usage_leaves_slices_and_tuples_unresolved() {
        assert_eq!(self_type_of("impl Tr for [UnixTime] {}"), None);
        assert_eq!(self_type_of("impl Tr for &[UnixTime] {}"), None);
        assert_eq!(self_type_of("impl Tr for (UnixTime, UnixTime) {}"), None);
        assert_eq!(self_type_of("impl Tr for () {}"), None);
    }

    #[test]
    fn test_usage_generic_parameter_is_a_path_segment() {
        assert_eq!(self_type_of("impl<T> Tr for T {}"), Some("T".to_string()));
        assert_eq!(self_type_of("impl<T> Tr for &T {}"), Some("T".to_string()));
    }
}

// no test_usage necessary
