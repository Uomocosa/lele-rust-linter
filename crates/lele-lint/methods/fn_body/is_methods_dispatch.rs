use crate::FnBody;

pub fn is_methods_dispatch(block: &FnBody) -> bool {
    let Some(segments) = block.single_path_call_segments() else {
        return false;
    };
    if segments.len() < 3 {
        return false;
    }
    let Some(first) = segments.first() else {
        return false;
    };
    if matches!(first.as_str(), "Self" | "self") {
        return false;
    }
    segments
        .iter()
        .rev()
        .skip(1)
        .any(|segment| segment.as_str() == "methods")
}

#[cfg(test)]
mod tests {
    use super::is_methods_dispatch;
    use crate::FnBody;
    use syn::Block;

    #[test]
    fn test_usage() {
        let crate_rooted: Block =
            syn::parse_str("{ crate::methods::foo::check(self, project) }").unwrap();
        assert!(is_methods_dispatch(&FnBody(&crate_rooted)));

        let bare: Block = syn::parse_str("{ methods::foo::check(self) }").unwrap();
        assert!(is_methods_dispatch(&FnBody(&bare)));

        let old_style: Block =
            syn::parse_str("{ checkers::foo_check::check(self, project) }").unwrap();
        assert!(!is_methods_dispatch(&FnBody(&old_style)));

        let short: Block = syn::parse_str("{ foo(x) }").unwrap();
        assert!(!is_methods_dispatch(&FnBody(&short)));

        let const_path: Block = syn::parse_str("{ Self::NAME }").unwrap();
        assert!(!is_methods_dispatch(&FnBody(&const_path)));

        let with_let: Block = syn::parse_str("{ let a = 1; foo(a) }").unwrap();
        assert!(!is_methods_dispatch(&FnBody(&with_let)));
    }
}
