use crate::Project;

pub fn layout_root_names(project: &Project) -> Vec<String> {
    project
        .layout_roots
        .iter()
        .map(|root| {
            root.file_name()
                .and_then(|name| name.to_str())
                .map_or_else(|| root.to_string_lossy().into_owned(), str::to_string)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::layout_root_names;
    use crate::Project;

    #[test]
    fn test_usage() {
        let project = Project {
            layout_roots: vec![PathBuf::from("src"), PathBuf::from("ui")],
            ..Project::default()
        };
        assert_eq!(
            layout_root_names(&project),
            vec!["src".to_string(), "ui".to_string()]
        );
    }
}
