use crate::rules;
use crate::Project;
use crate::RuleViolation;

const REQUIRED_LINTS: &[&str] = &[
    "unwrap_used",
    "expect_used",
    "indexing_slicing",
    "arithmetic_side_effects",
    "unreachable",
    "unimplemented",
    "unchecked_time_subtraction",
    "todo",
    "string_slice",
    "panic_in_result_fn",
    "panic",
    "exit",
    "as_conversions",
];

pub fn check(
    _self: &rules::clippy_config_cargo::ClippyConfigCargo,
    project: &Project,
) -> Vec<RuleViolation> {
    let cargo_path = project.root.join("Cargo.toml");
    let content = match std::fs::read_to_string(&cargo_path) {
        Ok(c) => c,
        Err(_) => {
            return vec![RuleViolation::from_file(cargo_path, rules::clippy_config_cargo::ClippyConfigCargo::CODE, "Cargo.toml not found or unreadable — add [lints.clippy] with minimum clippy config".to_string())];
        }
    };
    let value: toml::Value = match content.parse() {
        Ok(v) => v,
        Err(e) => {
            return vec![RuleViolation::from_file(
                cargo_path,
                rules::clippy_config_cargo::ClippyConfigCargo::CODE,
                format!("Cargo.toml parse error: {e}"),
            )];
        }
    };
    let clippy_table = rules::clippy_config_cargo::ClippyConfigCargo::resolve_clippy_table(
        &value,
        project,
        &cargo_path,
    );
    match clippy_table {
        Some(table) => {
            let mut violations = Vec::new();
            for key in ["pedantic", "nursery"] {
                match table.get(key) {
                    Some(v) if rules::clippy_config_cargo::ClippyConfigCargo::is_deny_with_priority(v, true) => {}
                    _ => violations.push(RuleViolation::from_file(cargo_path.clone(), rules::clippy_config_cargo::ClippyConfigCargo::CODE, format!("Cargo.toml `[lints.clippy].{key}` is missing or weaker than required — set it to {}", "`{ level = \"deny\", priority = -1 }`"))),
                }
            }
            for key in REQUIRED_LINTS {
                match table.get(*key) {
                    Some(v) if rules::clippy_config_cargo::ClippyConfigCargo::is_deny_with_priority(v, false) => {}
                    _ => violations.push(RuleViolation::from_file(cargo_path.clone(), rules::clippy_config_cargo::ClippyConfigCargo::CODE, format!("Cargo.toml `[lints.clippy].{key}` is missing or weaker than required — set it to {}", "`\"deny\"`"))),
                }
            }
            violations
        }
        None => vec![RuleViolation::from_file(cargo_path.clone(), rules::clippy_config_cargo::ClippyConfigCargo::CODE, "missing [lints.clippy] in Cargo.toml — add minimum clippy config (pedantic/nursery + 13 deny lints)".to_string())],
    }
}

#[cfg(test)]
mod tests {
    use crate::rules;

    #[test]
    fn test_usage() {
        let deny_str = toml::Value::String("deny".to_string());
        assert!(
            rules::clippy_config_cargo::ClippyConfigCargo::is_deny_with_priority(&deny_str, false)
        );
        let table: toml::Value =
            r#"value = { level = "deny", priority = -1 }"#.parse::<toml::Value>().unwrap()["value"]
                .clone();
        assert!(rules::clippy_config_cargo::ClippyConfigCargo::is_deny_with_priority(&table, true));
        let bad: toml::Value =
            r#"value = { level = "warn", priority = -1 }"#.parse::<toml::Value>().unwrap()["value"]
                .clone();
        assert!(!rules::clippy_config_cargo::ClippyConfigCargo::is_deny_with_priority(&bad, true));
    }
}
