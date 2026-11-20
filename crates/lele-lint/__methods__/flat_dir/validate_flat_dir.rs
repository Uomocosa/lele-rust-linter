use crate::rules;
use crate::FlatDirEntry;
use crate::RuleViolation;
use std::path::Path;

pub fn validate_flat_dir(flat: &FlatDirEntry, base: &Path) -> Vec<RuleViolation> {
    let mut violations = Vec::new();
    if flat.path.as_os_str().is_empty() {
        violations.push(RuleViolation::from_file(
            base.join("lele.toml"),
            rules::flat_dir::FlatDir::CODE,
            "flat_dir entry names no path".to_string(),
        ));
        return violations;
    }
    if !base.join(&flat.path).is_dir() {
        violations.push(RuleViolation::from_file(
            base.join("lele.toml"),
            rules::flat_dir::FlatDir::CODE,
            format!("flat_dir `{}` does not exist", flat.path.display(),),
        ));
    }
    violations
}

#[cfg(test)]
mod tests {
    use super::validate_flat_dir;
    use crate::FlatDirEntry;
    use std::path::PathBuf;

    #[test]
    fn test_usage() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(dir.path().join("src/flat")).unwrap();
        let present = FlatDirEntry {
            path: PathBuf::from("src/flat"),
            reason: "task".to_string(),
        };
        assert!(validate_flat_dir(&present, dir.path()).is_empty());
        let missing = FlatDirEntry {
            path: PathBuf::from("src/absent"),
            reason: "task".to_string(),
        };
        let violations = validate_flat_dir(&missing, dir.path());
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message.contains("src/absent"));
    }

    #[test]
    fn test_usage_empty_path_is_a_violation() {
        let dir = tempfile::tempdir().unwrap();
        let empty = FlatDirEntry {
            path: PathBuf::new(),
            reason: "task".to_string(),
        };
        assert_eq!(validate_flat_dir(&empty, dir.path()).len(), 1);
    }
}
