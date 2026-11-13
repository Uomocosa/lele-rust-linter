use std::path::Path;

pub fn parent_dir(rel: &Path) -> Option<String> {
    rel.parent()
        .and_then(Path::file_name)
        .and_then(|name| name.to_str())
        .map(str::to_string)
}

#[cfg(test)]
mod tests {
    use crate::rules;
    use std::path::Path;

    #[test]
    fn test_usage() {
        assert_eq!(
            rules::methods_layout::MethodsLayout::parent_dir(Path::new("src/thing/mod.rs")),
            Some("thing".to_string())
        );
        assert!(rules::methods_layout::MethodsLayout::parent_dir(Path::new("mod.rs")).is_none());
    }
}
