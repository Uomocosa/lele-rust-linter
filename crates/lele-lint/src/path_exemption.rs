use crate::Project;
use crate::RuleViolation;

pub struct PathExemption {
    pub code: String,
    pub root: std::path::PathBuf,
    pub reason: String,
    pub replaced_by: Option<String>,
}

impl PathExemption {
    pub(crate) fn matches(&self, violation: &RuleViolation, project: &Project) -> bool {
        let is_under_root = violation
            .file
            .starts_with(project.config_base().join(&self.root))
            || violation.file.starts_with(&self.root);
        violation.code == self.code && is_under_root
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::PathExemption;
    use crate::Project;
    use crate::RuleViolation;

    #[test]
    fn test_usage() {
        let project = Project {
            root: PathBuf::from("/crate"),
            ..Project::default()
        };
        let exemption = PathExemption {
            code: String::from("LELE012"),
            root: PathBuf::from("__rules__"),
            reason: String::from("rule sources do not follow the type naming rule"),
            replaced_by: None,
        };
        let under = RuleViolation::from_file(
            PathBuf::from("/crate/__rules__/r.rs"),
            "LELE012",
            String::from("m"),
        );
        let relative = RuleViolation::from_file(
            PathBuf::from("__rules__/r.rs"),
            "LELE012",
            String::from("m"),
        );
        let elsewhere = RuleViolation::from_file(
            PathBuf::from("/crate/src/a.rs"),
            "LELE012",
            String::from("m"),
        );
        let other_code = RuleViolation::from_file(
            PathBuf::from("/crate/__rules__/r.rs"),
            "LELECORE001",
            String::from("m"),
        );
        assert!(exemption.matches(&under, &project));
        assert!(exemption.matches(&relative, &Project::default()));
        assert!(!exemption.matches(&elsewhere, &project));
        assert!(!exemption.matches(&other_code, &project));
        let via_config_base = Project {
            root: PathBuf::from("/repo/crates/x"),
            config_dir: PathBuf::from("/repo"),
            ..Project::default()
        };
        let via_config = RuleViolation::from_file(
            PathBuf::from("/repo/crates/x/legacy/r.rs"),
            "LELE012",
            String::from("m"),
        );
        let config_exemption = PathExemption {
            code: String::from("LELE012"),
            root: PathBuf::from("crates/x/legacy"),
            reason: String::from("mirror"),
            replaced_by: Some(String::from("test_usage")),
        };
        assert!(config_exemption.matches(&via_config, &via_config_base));
    }
}
