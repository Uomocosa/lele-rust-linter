use std::collections::BTreeSet;
use std::path::PathBuf;

pub(crate) fn path_sorted_set(paths: &[&str]) -> BTreeSet<PathBuf> {
    paths.iter().map(PathBuf::from).collect()
}

#[cfg(test)]
mod tests {
    use super::path_sorted_set;
    use std::path::PathBuf;

    #[test]
    fn test_usage() {
        assert!(path_sorted_set(&["a.rs"]).contains(&PathBuf::from("a.rs")));
    }
}
