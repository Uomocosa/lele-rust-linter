use std::path::Path;

pub fn parent_dir_name(rel_path: &Path) -> Option<String> {
    rel_path
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .and_then(|p| p.file_name())
        .and_then(|n| n.to_str())
        .map(str::to_string)
}

#[cfg(test)]
mod tests {
    use crate::rules;
    use std::path::Path;

    #[test]
    fn test_usage() {
        assert_eq!(
            rules::no_stuttered_type::NoStutteredType::parent_dir_name(Path::new(
                "freenet/client.rs"
            )),
            Some("freenet".to_string())
        );
        assert!(
            rules::no_stuttered_type::NoStutteredType::parent_dir_name(Path::new("lib.rs"))
                .is_none()
        );
    }
}
