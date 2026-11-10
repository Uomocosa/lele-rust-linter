use crate::common;

pub fn module_path(path: &common::SourcePath) -> Vec<String> {
    let mut components: Vec<String> = path
        .components()
        .filter_map(|c| c.as_os_str().to_str().map(str::to_string))
        .collect();
    let Some(file_name) = components.pop() else {
        return Vec::new();
    };
    if path.is_index_file() {
        return components;
    }
    let stem = file_name.strip_suffix(".rs").unwrap_or(&file_name);
    components.push(stem.to_string());
    components
}

#[cfg(test)]
mod tests {
    use super::module_path;
    use crate::common;
    use std::path::PathBuf;

    #[test]
    fn test_usage() {
        assert_eq!(
            module_path(&common::SourcePath(&PathBuf::from("a/mod.rs"))),
            vec!["a".to_string()]
        );
        assert_eq!(
            module_path(&common::SourcePath(&PathBuf::from("a/b.rs"))),
            vec!["a".to_string(), "b".to_string()]
        );
    }
}
