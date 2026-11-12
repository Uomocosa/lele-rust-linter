use crate::scan;

pub fn plugin_types(file: &scan::ScannedFile) -> Vec<String> {
    file.items
        .iter()
        .filter_map(|item| match item {
            syn::Item::Impl(impl_block) => scan::PluginImpl(impl_block).plugin_type(),
            _ => None,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::plugin_types;
    use crate::scan;

    #[test]
    fn test_usage() {
        let file: syn::File = syn::parse_quote! {
            impl Plugin for GamePlugin { fn build(&self, app: &mut App) {} }
            impl Helper for Thing {}
        };
        assert_eq!(
            plugin_types(&scan::ScannedFile(&file)),
            vec!["GamePlugin".to_string()]
        );
    }
}
