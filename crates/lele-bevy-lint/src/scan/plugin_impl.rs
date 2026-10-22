use atomic_delegate_macros::atomic_delegates;
use derive_more::Deref;

#[derive(Clone, Copy, Deref)]
pub struct PluginImpl<'a>(pub &'a syn::ItemImpl);

#[atomic_delegates]
impl PluginImpl<'_> {
    pub fn plugin_type(&self) -> Option<String> {}
}

#[cfg(test)]
mod tests {
    use super::PluginImpl;

    #[test]
    fn test_usage() {
        let plugin: syn::ItemImpl = syn::parse_str(
            "impl bevy::prelude::Plugin for LobbyUi { fn build(&self, _a: &mut A) {} }",
        )
        .unwrap();
        assert_eq!(
            PluginImpl(&plugin).plugin_type(),
            Some("LobbyUi".to_string())
        );
    }
}
