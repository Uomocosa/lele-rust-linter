use crate::rules;

pub fn is_allowed(item: &syn::Item) -> bool {
    let is_import_or_constant = matches!(item, syn::Item::Use(_) | syn::Item::Const(_));
    let is_test_function = match item {
        syn::Item::Fn(function) => rules::test_module_items::TestModuleItems::is_test_fn(function),
        _ => false,
    };
    is_import_or_constant || is_test_function
}

#[cfg(test)]
mod tests {
    use super::is_allowed;
    use crate::rules;

    #[test]
    fn test_usage() {
        let import: syn::Item = syn::parse_quote! { use crate::stock; };
        assert!(is_allowed(&import));
        let constant: syn::Item = syn::parse_quote! { const LIMIT: usize = 3; };
        assert!(is_allowed(&constant));
        let test: syn::Item = syn::parse_quote! { #[test] fn test_usage() {} };
        assert!(is_allowed(&test));
        assert!(rules::test_module_items::TestModuleItems::is_allowed(&test));
        let helper: syn::Item = syn::parse_quote! { fn make_guest() {} };
        assert!(!is_allowed(&helper));
        let nested: syn::Item = syn::parse_quote! { mod inner {} };
        assert!(!is_allowed(&nested));
        let implementation: syn::Item = syn::parse_quote! { impl Guest { pub fn id(&self) {} } };
        assert!(!is_allowed(&implementation));
    }
}
