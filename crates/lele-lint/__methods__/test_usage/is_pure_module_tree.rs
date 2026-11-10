pub fn is_pure_module_tree(file: &syn::File) -> bool {
    if file.items.is_empty() {
        return true;
    }
    file.items.iter().all(|item| {
        matches!(
            item,
            syn::Item::Mod(_) | syn::Item::Use(_) | syn::Item::Macro(_)
        )
    })
}

// no test_usage necessary
