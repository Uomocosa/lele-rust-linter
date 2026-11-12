use crate::Project;

pub fn methods_root_name(project: &Project) -> String {
    project
        .methods_dir
        .as_ref()
        .and_then(|dir| dir.file_name())
        .and_then(|name| name.to_str())
        .unwrap_or("__methods__")
        .to_string()
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::methods_root_name;
    use crate::Project;

    #[test]
    fn test_usage() {
        assert_eq!(methods_root_name(&Project::default()), "__methods__");
        let named = Project {
            methods_dir: Some(PathBuf::from("custom_methods")),
            ..Project::default()
        };
        assert_eq!(methods_root_name(&named), "custom_methods");
    }
}
