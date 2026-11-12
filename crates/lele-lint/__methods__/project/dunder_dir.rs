use crate::Project;

pub fn dunder_dir(project: &Project) -> Option<String> {
    project
        .dunder_paths
        .iter()
        .find(|dunder| dunder.is_dunder_dir())
        .and_then(|dunder| dunder.file_name())
        .and_then(|name| name.to_str())
        .map(String::from)
}

#[cfg(test)]
mod tests {
    use super::dunder_dir;
    use crate::DunderPath;
    use crate::Project;

    #[test]
    fn test_usage() {
        let project = Project {
            dunder_paths: DunderPath::defaults(),
            ..Project::default()
        };
        assert_eq!(dunder_dir(&project).as_deref(), Some("__basic__"));
        assert_eq!(dunder_dir(&Project::default()), None);
    }
}
