use std::path::Path;

pub fn file_stem(rel: &Path) -> Option<String> {
    rel.file_stem()
        .and_then(|stem| stem.to_str())
        .map(str::to_string)
}

#[cfg(test)]
mod tests {
    use crate::rules;
    use std::path::Path;

    #[test]
    fn test_usage() {
        assert_eq!(
            rules::methods_layout::MethodsLayout::file_stem(Path::new("src/thing/run.rs")),
            Some("run".to_string())
        );
    }
}
