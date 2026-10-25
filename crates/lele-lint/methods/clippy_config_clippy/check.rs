use crate::rules;
use crate::Project;
use crate::RuleViolation;

const REQUIRED_KEYS: &[&str] = &[
    "allow-unwrap-in-tests",
    "allow-expect-in-tests",
    "allow-panic-in-tests",
    "allow-indexing-slicing-in-tests",
];

pub fn check(
    _self: &rules::clippy_config_clippy::ClippyConfigClippy,
    project: &Project,
) -> Vec<RuleViolation> {
    let path = project.config_base().join("clippy.toml");
    let content = match std::fs::read_to_string(&path) {
        Ok(c) => c,
        Err(_) => {
            return vec![RuleViolation::from_file(path, rules::clippy_config_clippy::ClippyConfigClippy::CODE, "clippy.toml not found — add it with allow-unwrap-in-tests, allow-expect-in-tests, allow-panic-in-tests, allow-indexing-slicing-in-tests = true".to_string())];
        }
    };
    let value: toml::Value = match content.parse() {
        Ok(v) => v,
        Err(e) => {
            return vec![RuleViolation::from_file(
                path,
                rules::clippy_config_clippy::ClippyConfigClippy::CODE,
                format!("clippy.toml parse error: {e}"),
            )];
        }
    };
    let table = match value.as_table() {
        Some(t) => t,
        None => {
            return vec![RuleViolation::from_file(
                path,
                rules::clippy_config_clippy::ClippyConfigClippy::CODE,
                "clippy.toml must be a table with allow-* = true entries".to_string(),
            )];
        }
    };
    let mut violations = Vec::new();
    for key in REQUIRED_KEYS {
        match table.get(*key) {
            Some(toml::Value::Boolean(true)) => {}
            _ => violations.push(RuleViolation::from_file(
                path.clone(),
                rules::clippy_config_clippy::ClippyConfigClippy::CODE,
                format!("clippy.toml is missing `{key}` — add `{key} = true`"),
            )),
        }
    }
    violations
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_usage() {
        let content = r"
allow-unwrap-in-tests = true
allow-expect-in-tests = true
allow-panic-in-tests = true
allow-indexing-slicing-in-tests = true
";
        let value: toml::Value = content.parse().unwrap();
        assert!(value.get("allow-unwrap-in-tests").is_some());
    }
}
