pub fn has_underscore_tests_stem(path: &std::path::Path) -> bool {
    path.extension().is_some_and(|ext| ext == "rs")
        && path
            .file_stem()
            .and_then(|stem| stem.to_str())
            .is_some_and(|stem| stem.ends_with("_tests"))
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::rules;

    #[test]
    fn test_usage() {
        assert!(rules::test_inline::TestInline::has_underscore_tests_stem(
            Path::new("src/greet_tests.rs")
        ));
        assert!(!rules::test_inline::TestInline::has_underscore_tests_stem(
            Path::new("src/greet.rs")
        ));
        assert!(!rules::test_inline::TestInline::has_underscore_tests_stem(
            Path::new("src/tests.rs")
        ));
        assert!(!rules::test_inline::TestInline::has_underscore_tests_stem(
            Path::new("src/greet_tests.txt")
        ));
    }
}
