use crate::rules;
use crate::testing;

pub(crate) fn error_message_violation_count(src: &str) -> usize {
    testing::violation_messages(
        &rules::error_message_omits_source::ErrorMessageOmitsSource,
        &[("error.rs", src)],
    )
    .len()
}

#[cfg(test)]
mod tests {
    use super::error_message_violation_count;

    #[test]
    fn test_usage() {
        assert_eq!(
            error_message_violation_count(
                "#[derive(Debug, thiserror::Error)]\npub enum Error {\n    #[error(\"x\")]\n    Io(#[from] std::io::Error),\n}\n"
            ),
            0
        );
    }
}
