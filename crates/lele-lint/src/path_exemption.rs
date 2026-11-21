use crate::Project;
use crate::RuleViolation;

pub struct PathExemption {
    pub code: &'static str,
    pub root: &'static str,
    pub reason: &'static str,
}

impl PathExemption {
    pub(crate) fn matches(&self, violation: &RuleViolation, project: &Project) -> bool {
        let is_under_root = violation.file.starts_with(project.root.join(self.root))
            || violation.file.starts_with(self.root);
        self.code == violation.code && is_under_root
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
            code: "LELE012",
            root: "__rules__",
            reason: "rule sources do not follow the type naming rule",
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
    }
}
