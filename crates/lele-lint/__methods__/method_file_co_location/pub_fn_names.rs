pub fn pub_fn_names(file: &syn::File) -> Vec<String> {
    file.items
        .iter()
        .filter_map(|item| match item {
            syn::Item::Fn(f) if matches!(f.vis, syn::Visibility::Public(_)) => {
                Some(f.sig.ident.to_string())
            }
            _ => None,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::rules;

    #[test]
    fn test_usage() {
        let file: syn::File = syn::parse_quote! { pub fn run() {} fn hidden() {} };
        assert_eq!(
            rules::method_file_co_location::MethodFileCoLocation::pub_fn_names(&file),
            ["run"]
        );
    }
}
