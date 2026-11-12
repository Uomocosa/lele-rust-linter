use crate::scan;

pub fn module_fns_of(items: &[syn::Item]) -> scan::ModuleFns<'_> {
    items
        .iter()
        .filter_map(|item| match item {
            syn::Item::Fn(func) => Some((func.sig.ident.to_string(), func)),
            _ => None,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::module_fns_of;

    #[test]
    fn test_usage() {
        let file: syn::File = syn::parse_quote! {
            fn build() {}
            struct Plain;
        };
        let found = module_fns_of(&file.items);
        assert_eq!(found.len(), 1);
        assert!(found.contains_key("build"));
    }
}
