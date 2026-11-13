pub fn has_tests_component(path: &std::path::Path) -> bool {
    path.components()
        .any(|c| c.as_os_str().to_str().is_some_and(|s| s == "tests"))
}

#[cfg(test)]
mod tests {
    use crate::rules;
    use std::path::Path;

    #[test]
    fn test_usage() {
        assert!(rules::test_inline::TestInline::has_tests_component(
            Path::new("src/tests/a.rs")
        ));
        assert!(!rules::test_inline::TestInline::has_tests_component(
            Path::new("src/a.rs")
        ));
    }
}
