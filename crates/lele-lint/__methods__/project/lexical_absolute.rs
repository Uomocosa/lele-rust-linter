use std::path::Component;
use std::path::Path;
use std::path::PathBuf;

pub fn lexical_absolute(path: &Path) -> PathBuf {
    let absolute = std::path::absolute(path).unwrap_or_else(|_| path.to_path_buf());
    let mut out = PathBuf::new();
    for component in absolute.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                let _ = out.pop();
            }
            _ => out.push(component.as_os_str()),
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use rstest::rstest;

    use super::lexical_absolute;

    #[test]
    fn test_usage() {
        let resolved = lexical_absolute(Path::new("/srv/data/file.rs"));
        assert_eq!(resolved, std::path::PathBuf::from("/srv/data/file.rs"));
        assert!(resolved.is_absolute());
    }

    #[rstest]
    #[case("a/./b", "a/b")]
    #[case("a/b/../c", "a/c")]
    #[case("/already/absolute", "/already/absolute")]
    #[case("/nonexistent-lele-lint-probe/sub", "/nonexistent-lele-lint-probe/sub")]
    fn lexical_absolute_normalizes_lexically(#[case] raw: &str, #[case] tail: &str) {
        let resolved = lexical_absolute(Path::new(raw));
        assert!(resolved.ends_with(tail), "{resolved:?}");
        assert!(resolved.is_absolute(), "{resolved:?}");
    }
}
