use crate::Project;
use crate::RelaxedRule;

pub fn applies_to(rule: &RelaxedRule, project: &Project) -> bool {
    match rule {
        RelaxedRule::Off { .. } => true,
        RelaxedRule::Exempt { path, .. } => {
            let resolved = Project::lexical_absolute(&project.config_base().join(path));
            let root = Project::lexical_absolute(&project.root);
            resolved.starts_with(&root) || root.starts_with(&resolved)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::applies_to;
    use crate::Project;
    use crate::RelaxedRule;

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
        assert!(applies_to(&off, &project));
        let under = RelaxedRule::Exempt {
            name: String::from("no_comments"),
            code: String::from("LELECORE015"),
            path: std::path::PathBuf::from("src/legacy"),
            reason: String::from("mirror"),
            replaced_by: None,
        };
        assert!(applies_to(&under, &project));
    }
}
