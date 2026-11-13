pub fn defines_pub_fn(file: &syn::File, name: &str) -> bool {
    file.items.iter().any(|item| {
        matches!(
            item,
            syn::Item::Fn(func)
                if matches!(func.vis, syn::Visibility::Public(_))
                    && func.sig.ident == name
        )
    })
}

#[cfg(test)]
mod tests {
    use crate::rules;

    #[test]
    fn test_usage() {
        let file: syn::File = syn::parse_quote! { pub fn run() {} fn hidden() {} };
        assert!(rules::methods_layout::MethodsLayout::defines_pub_fn(
            &file, "run"
        ));
        assert!(!rules::methods_layout::MethodsLayout::defines_pub_fn(
            &file, "hidden"
        ));
    }
}
