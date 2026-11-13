pub fn is_deny_with_priority(value: &toml::Value, require_priority: bool) -> bool {
    match value {
        toml::Value::String(s) => s == "deny",
        toml::Value::Table(t) => {
            let level_ok = t.get("level").and_then(|v| v.as_str()) == Some("deny");
            if !level_ok {
                return false;
            }
            if require_priority {
                match t.get("priority").and_then(|v| v.as_integer()) {
                    Some(-1) => true,
                    _ => false,
                }
            } else {
                true
            }
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use crate::rules;

    #[test]
    fn test_usage() {
        let string = toml::Value::String("deny".to_string());
        assert!(
            rules::clippy_config_cargo::ClippyConfigCargo::is_deny_with_priority(&string, false)
        );
        let with_priority: toml::Value = toml::from_str("level = \"deny\"\npriority = -1").unwrap();
        assert!(
            rules::clippy_config_cargo::ClippyConfigCargo::is_deny_with_priority(
                &with_priority,
                true
            )
        );
        let without_priority: toml::Value = toml::from_str("level = \"deny\"").unwrap();
        assert!(
            !rules::clippy_config_cargo::ClippyConfigCargo::is_deny_with_priority(
                &without_priority,
                true
            )
        );
        assert!(
            rules::clippy_config_cargo::ClippyConfigCargo::is_deny_with_priority(
                &without_priority,
                false
            )
        );
    }
}
