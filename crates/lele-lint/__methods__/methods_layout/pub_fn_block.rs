pub fn pub_fn_block<'a>(file: &'a syn::File, name: &str) -> Option<&'a syn::Block> {
    file.items.iter().find_map(|item| match item {
        syn::Item::Fn(func)
            if matches!(func.vis, syn::Visibility::Public(_)) && func.sig.ident == name =>
        {
            Some(&*func.block)
        }
        _ => None,
    })
}

#[cfg(test)]
mod tests {
    use crate::rules;

    #[test]
    fn test_usage() {
        let file: syn::File = syn::parse_quote! { pub fn run() -> u8 { 1 } };
        assert_eq!(
            rules::methods_layout::MethodsLayout::pub_fn_block(&file, "run")
                .map(|block| block.stmts.len()),
            Some(1)
        );
        assert!(rules::methods_layout::MethodsLayout::pub_fn_block(&file, "walk").is_none());
    }
}
