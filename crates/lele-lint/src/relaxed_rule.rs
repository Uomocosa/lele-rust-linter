use atomic_delegate_macros::atomic_delegates;

use crate::Project;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RelaxedRule {
    Off {
        name: String,
        code: String,
        reason: String,
    },
    Exempt {
        name: String,
        code: String,
        path: std::path::PathBuf,
        reason: String,
        replaced_by: Option<String>,
    },
}

#[atomic_delegates]
impl RelaxedRule {
    pub fn applies_to(&self, project: &Project) -> bool {}
}

#[cfg(test)]
mod tests {
    use super::RelaxedRule;
    use crate::Project;

    #[test]
    fn test_usage() {
        let dir = tempfile::tempdir().unwrap();
        let project = Project {
            root: dir.path().to_path_buf(),
            ..Project::default()
        };
        let off = RelaxedRule::Off {
            name: String::from("domain_import"),
            code: String::from("LELE016"),
            reason: String::from("legacy"),
        };
        assert!(off.applies_to(&project));
        let under = RelaxedRule::Exempt {
            name: String::from("no_comments"),
            code: String::from("LELECORE015"),
            path: std::path::PathBuf::from("src/legacy"),
            reason: String::from("mirror"),
            replaced_by: None,
        };
        assert!(under.applies_to(&project));
        let other = tempfile::tempdir().unwrap();
        let outside = RelaxedRule::Exempt {
            name: String::from("no_comments"),
            code: String::from("LELECORE015"),
            path: other.path().join("x"),
            reason: String::from("other crate"),
            replaced_by: None,
        };
        assert!(!outside.applies_to(&project));
        let covering = RelaxedRule::Exempt {
            name: String::from("no_comments"),
            code: String::from("LELECORE015"),
            path: std::path::PathBuf::from(""),
            reason: String::from("whole tree"),
            replaced_by: None,
        };
        assert!(covering.applies_to(&project));
    }
}
