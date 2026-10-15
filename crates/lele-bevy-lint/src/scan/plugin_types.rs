use crate::scan;

#[must_use]
pub fn plugin_types(file: &syn::File) -> Vec<String> {
    file.items
        .iter()
        .filter_map(|item| match item {
            syn::Item::Impl(impl_block) => scan::plugin_type_of(impl_block),
            _ => None,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::plugin_types;

    #[test]
    fn test_usage() {
        let file = syn::parse_str(
            "impl bevy::prelude::Plugin for LobbyUi { fn build(&self, _a: &mut A) {} }
             impl LobbyUi { fn helper(&self) {} }",
        )
        .unwrap();
        assert_eq!(plugin_types(&file), vec!["LobbyUi".to_string()]);
    }
}
