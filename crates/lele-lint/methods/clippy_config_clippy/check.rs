use std::path::PathBuf;

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
    let mut found: Option<PathBuf> = None;
    let mut dir = Some(project.root.as_path());
    while let Some(current) = dir {
        let plain = current.join("clippy.toml");
        if plain.is_file() {
            found = Some(plain);
            break;
        }
        let dotted = current.join(".clippy.toml");
        if dotted.is_file() {
            found = Some(dotted);
            break;
        }
        dir = current.parent();
    }
    let Some(path) = found else {
        let missing = project.root.join("clippy.toml");
        return vec![RuleViolation::from_file(missing, rules::clippy_config_clippy::ClippyConfigClippy::CODE, "clippy.toml not found — add it with allow-unwrap-in-tests, allow-expect-in-tests, allow-panic-in-tests, allow-indexing-slicing-in-tests = true".to_string())];
    };
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
    use crate::rules;
    use crate::Project;

    const VALID_CLIPPY_TOML: &str = "allow-unwrap-in-tests = true\nallow-expect-in-tests = true\nallow-panic-in-tests = true\nallow-indexing-slicing-in-tests = true\n";

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

    #[test]
    fn test_parent_clippy_toml_is_used() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("clippy.toml"), VALID_CLIPPY_TOML).unwrap();
        let child = dir.path().join("member");
        std::fs::create_dir_all(&child).unwrap();
        let project = Project {
            root: child,
            ..Project::default()
        };
        let rule = rules::clippy_config_clippy::ClippyConfigClippy;
        let violations = super::check(&rule, &project);
        assert!(violations.is_empty());
    }

    #[test]
    fn test_parent_invalid_clippy_toml_is_reported_at_parent_path() {
        let dir = tempfile::tempdir().unwrap();
        let parent_file = dir.path().join("clippy.toml");
        std::fs::write(&parent_file, "allow-unwrap-in-tests = true\n").unwrap();
        let child = dir.path().join("member");
        std::fs::create_dir_all(&child).unwrap();
        let project = Project {
            root: child,
            ..Project::default()
        };
        let rule = rules::clippy_config_clippy::ClippyConfigClippy;
        let violations = super::check(&rule, &project);
        assert!(!violations.is_empty());
        assert!(violations
            .iter()
            .all(|violation| violation.file == parent_file));
    }

    #[test]
    fn test_crate_clippy_toml_wins_over_parent() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(
            dir.path().join("clippy.toml"),
            "allow-unwrap-in-tests = true\n",
        )
        .unwrap();
        let child = dir.path().join("member");
        std::fs::create_dir_all(&child).unwrap();
        std::fs::write(child.join("clippy.toml"), VALID_CLIPPY_TOML).unwrap();
        let project = Project {
            root: child.clone(),
            ..Project::default()
        };
        let rule = rules::clippy_config_clippy::ClippyConfigClippy;
        let violations = super::check(&rule, &project);
        assert!(violations.is_empty());
        std::fs::write(child.join("clippy.toml"), "allow-unwrap-in-tests = true\n").unwrap();
        std::fs::write(dir.path().join("clippy.toml"), VALID_CLIPPY_TOML).unwrap();
        let violations = super::check(&rule, &project);
        assert!(!violations.is_empty());
        assert!(violations
            .iter()
            .all(|violation| violation.file == child.join("clippy.toml")));
    }

    #[test]
    fn test_missing_everywhere_reports_missing() {
        let dir = tempfile::tempdir().unwrap();
        let child = dir.path().join("member");
        std::fs::create_dir_all(&child).unwrap();
        let project = Project {
            root: child,
            ..Project::default()
        };
        let rule = rules::clippy_config_clippy::ClippyConfigClippy;
        let violations = super::check(&rule, &project);
        assert_eq!(violations.len(), 1);
        assert!(violations
            .first()
            .is_some_and(|violation| violation.message.contains("not found")));
    }

    #[test]
    fn test_dot_clippy_toml_in_parent_is_used() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join(".clippy.toml"), VALID_CLIPPY_TOML).unwrap();
        let child = dir.path().join("member");
        std::fs::create_dir_all(&child).unwrap();
        let project = Project {
            root: child,
            ..Project::default()
        };
        let rule = rules::clippy_config_clippy::ClippyConfigClippy;
        let violations = super::check(&rule, &project);
        assert!(violations.is_empty());
    }
}
