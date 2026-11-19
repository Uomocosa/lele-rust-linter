use std::path::PathBuf;

use crate::DunderPath;
use crate::Error;

pub fn try_from_path_buf(path: PathBuf) -> Result<DunderPath, Error> {
    let is_dunder = path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .and_then(|stem| stem.strip_prefix("__"))
        .and_then(|rest| rest.strip_suffix("__"))
        .is_some_and(|inner| !inner.is_empty());
    if !is_dunder {
        return Err(Error::NotDunder(path));
    }
    Ok(DunderPath(path))
}

#[cfg(test)]
mod tests {
    use crate::testing;

    #[test]
    fn test_usage() {
        assert!(testing::parses_as_dunder("__basic__"));
        assert!(testing::parses_as_dunder("__prelude__.rs"));
        assert!(testing::parses_as_dunder("__notes__.md"));
        assert!(testing::parses_as_dunder("src/__basic__"));
    }

    #[test]
    fn test_usage_rejects_non_dunder_names() {
        assert!(!testing::parses_as_dunder("basic"));
        assert!(!testing::parses_as_dunder("__basic"));
        assert!(!testing::parses_as_dunder("basic__"));
        assert!(!testing::parses_as_dunder("____"));
        assert!(!testing::parses_as_dunder("____.rs"));
        assert!(!testing::parses_as_dunder("__basic__/enums.rs"));
    }
}
