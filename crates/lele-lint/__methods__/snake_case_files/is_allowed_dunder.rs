use crate::Project;

pub fn is_allowed_dunder(project: &Project, name: &str) -> bool {
    project
        .dunder_paths
        .iter()
        .any(|dunder| dunder.file_stem() == Some(std::ffi::OsStr::new(name)))
}

#[cfg(test)]
mod tests {
    use crate::rules;
    use crate::DunderPath;
    use crate::Project;

    #[test]
    fn test_usage() {
        let project = Project {
            dunder_paths: DunderPath::defaults(),
            ..Project::default()
        };
        assert!(rules::snake_case_files::SnakeCaseFiles::is_allowed_dunder(
            &project,
            "__basic__"
        ));
        assert!(!rules::snake_case_files::SnakeCaseFiles::is_allowed_dunder(
            &project, "stock"
        ));
    }
}
