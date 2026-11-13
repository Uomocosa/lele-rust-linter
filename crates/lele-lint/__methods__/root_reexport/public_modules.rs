use std::collections::HashSet;

pub fn public_modules(lib: &syn::File) -> HashSet<String> {
    lib.items
        .iter()
        .filter_map(|item| match item {
            syn::Item::Mod(m) if matches!(m.vis, syn::Visibility::Public(_)) => {
                Some(m.ident.to_string())
            }
            _ => None,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::rules;
    use std::collections::HashSet;

    #[test]
    fn test_usage() {
        let lib: syn::File = syn::parse_quote! { pub mod stock; mod hidden; pub fn f() {} };
        assert_eq!(
            rules::root_reexport::RootReexport::public_modules(&lib),
            HashSet::from(["stock".to_string()])
        );
    }
}
