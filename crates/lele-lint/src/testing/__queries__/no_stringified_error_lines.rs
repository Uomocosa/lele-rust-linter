use crate::rules;
use crate::testing;

const ERROR_ENUM: &str = "#[derive(Debug, thiserror::Error)]\npub enum Error {\n    #[error(\"x\")]\n    Io(String),\n}\n";

pub(crate) fn no_stringified_error_lines(src: &str) -> Vec<usize> {
    testing::violation_lines(
        &rules::no_stringified_errors::NoStringifiedErrors,
        &[("error.rs", ERROR_ENUM), ("probe.rs", src)],
    )
}

#[cfg(test)]
mod tests {
    use super::no_stringified_error_lines;

    #[test]
    fn test_usage() {
        assert_eq!(
            no_stringified_error_lines("fn f() { a().map_err(|e| Error::Io(e.to_string())); }\n"),
            vec![1]
        );
    }
}
