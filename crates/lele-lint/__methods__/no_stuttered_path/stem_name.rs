use std::path::Path;

pub fn stem_name(rel_path: &Path) -> Option<String> {
    rel_path
        .file_stem()
        .and_then(|s| s.to_str())
        .map(str::to_string)
}

#[cfg(test)]
mod tests {
    use crate::rules;
    use std::path::Path;

    #[test]
    fn test_usage() {
        assert_eq!(
            rules::no_stuttered_path::NoStutteredPath::stem_name(Path::new("src/stock/item.rs")),
            Some("item".to_string())
        );
    }
}
