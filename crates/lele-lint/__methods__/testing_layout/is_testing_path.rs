use std::path::Path;

pub fn is_testing_path(rel_path: &Path) -> bool {
    let Some(first) = rel_path.components().next() else {
        return false;
    };
    first.as_os_str() == "testing"
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::is_testing_path;
    use crate::rules;

    #[test]
    fn test_usage() {
        assert!(is_testing_path(Path::new("testing/mod.rs")));
        assert!(is_testing_path(Path::new("testing/__fixtures__/row.rs")));
        assert!(rules::testing_layout::TestingLayout::is_testing_path(
            Path::new("testing/mod.rs")
        ));
        assert!(!is_testing_path(Path::new("stock/mod.rs")));
        assert!(!is_testing_path(Path::new("tests/mod.rs")));
        assert!(!is_testing_path(Path::new("testing.rs")));
    }
}
