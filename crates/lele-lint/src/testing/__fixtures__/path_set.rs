use std::collections::HashSet;
use std::path::PathBuf;

pub(crate) fn path_set(paths: &[&str]) -> HashSet<PathBuf> {
    paths.iter().map(PathBuf::from).collect()
}

#[cfg(test)]
mod tests {
    use super::path_set;
    use std::path::PathBuf;

    #[test]
    fn test_usage() {
        assert!(path_set(&["a.rs"]).contains(&PathBuf::from("a.rs")));
    }
}
