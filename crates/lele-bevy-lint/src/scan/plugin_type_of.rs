#[must_use]
pub fn plugin_type_of(impl_block: &syn::ItemImpl) -> Option<String> {
    let is_plugin = impl_block.trait_.as_ref().is_some_and(|(_, path, _)| {
        path.segments
            .last()
            .is_some_and(|segment| segment.ident == "Plugin")
    });
    if !is_plugin {
        return None;
    }
    match &*impl_block.self_ty {
        syn::Type::Path(path) => path
            .path
            .segments
            .last()
            .map(|segment| segment.ident.to_string()),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::plugin_type_of;

    #[test]
    fn test_usage() {
        let plugin: syn::ItemImpl = syn::parse_str(
            "impl bevy::prelude::Plugin for LobbyUi { fn build(&self, _a: &mut A) {} }",
        )
        .unwrap();
        let inherent: syn::ItemImpl =
            syn::parse_str("impl LobbyUi { fn helper(&self) {} }").unwrap();
        assert_eq!(plugin_type_of(&plugin), Some("LobbyUi".to_string()));
        assert_eq!(plugin_type_of(&inherent), None);
    }
}
