use crate::scan;

pub fn defines_plugin(file: &scan::ScannedFile) -> bool {
    file.items.iter().any(|item| match item {
        syn::Item::Impl(impl_block) => impl_block.trait_.as_ref().is_some_and(|(_, path, _)| {
            path.segments
                .last()
                .is_some_and(|segment| segment.ident == "Plugin")
        }),
        _ => false,
    })
}

#[cfg(test)]
mod tests {
    use super::defines_plugin;
    use crate::scan;

    #[test]
    fn test_usage() {
        let plugin: syn::File = syn::parse_quote! {
            impl Plugin for GamePlugin { fn build(&self, app: &mut App) {} }
        };
        let other: syn::File = syn::parse_quote! {
            impl Helper for Thing {}
        };
        assert!(defines_plugin(&scan::ScannedFile(&plugin)));
        assert!(!defines_plugin(&scan::ScannedFile(&other)));
    }
}
