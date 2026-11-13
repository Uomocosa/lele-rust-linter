pub fn inherits_workspace_lints(value: &toml::Value) -> bool {
    value
        .get("lints")
        .and_then(|v| v.get("workspace"))
        .and_then(|v| v.as_bool())
        == Some(true)
}

#[cfg(test)]
mod tests {
    use crate::rules;

    #[test]
    fn test_usage() {
        let inherits: toml::Value = toml::from_str("[lints]\nworkspace = true").unwrap();
        assert!(rules::clippy_config_cargo::ClippyConfigCargo::inherits_workspace_lints(&inherits));
        let own: toml::Value = toml::from_str("[lints.clippy]\npedantic = \"deny\"").unwrap();
        assert!(!rules::clippy_config_cargo::ClippyConfigCargo::inherits_workspace_lints(&own));
    }
}
