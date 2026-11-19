use std::path::PathBuf;

use crate::DunderPath;

pub(crate) fn parses_as_dunder(raw: &str) -> bool {
    DunderPath::try_from(PathBuf::from(raw)).is_ok()
}

#[cfg(test)]
mod tests {
    use super::parses_as_dunder;

    #[test]
    fn test_usage() {
        assert!(parses_as_dunder("__basic__"));
    }
}
