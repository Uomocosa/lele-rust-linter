pub struct RegistryPlugin;

impl Plugin for RegistryPlugin {
    fn build(&self, app: &mut App) {
        let plugin = RegistryPlugin;
        app.add(plugin);
    }
}

#[cfg(test)]
mod tests {
    use super::RegistryPlugin;

    #[test]
    fn test_usage() {
        let plugin = RegistryPlugin;
        assert!(matches!(plugin, RegistryPlugin));
    }
}
