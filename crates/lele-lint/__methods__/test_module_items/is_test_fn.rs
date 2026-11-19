use crate::ItemMarker;

pub fn is_test_fn(function: &syn::ItemFn) -> bool {
    for attr in &function.attrs {
        let marker = ItemMarker(attr);
        if marker.is_test() || marker.is_rstest() || marker.is_tokio_test() {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::is_test_fn;
    use crate::rules;

    #[test]
    fn test_usage() {
        let plain: syn::ItemFn = syn::parse_quote! { #[test] fn test_usage() {} };
        assert!(is_test_fn(&plain));
        assert!(rules::test_module_items::TestModuleItems::is_test_fn(
            &plain
        ));
        let rstest: syn::ItemFn =
            syn::parse_quote! { #[rstest] #[case(1)] fn case_one(#[case] value: u8) {} };
        assert!(is_test_fn(&rstest));
        let tokio_test: syn::ItemFn = syn::parse_quote! {
            #[tokio::test(flavor = "multi_thread")]
            async fn dials() {}
        };
        assert!(is_test_fn(&tokio_test));
        let helper: syn::ItemFn = syn::parse_quote! { fn make_guest() {} };
        assert!(!is_test_fn(&helper));
    }
}
