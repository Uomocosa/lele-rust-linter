use std::path::PathBuf;

use crate::FlatDirEntry;

pub(crate) fn flat_dir_entries() -> Vec<FlatDirEntry> {
    vec![FlatDirEntry {
        path: PathBuf::from("src/flat"),
        reason: "one file per item".to_string(),
    }]
}

#[cfg(test)]
mod tests {
    use super::flat_dir_entries;

    #[test]
    fn test_usage() {
        let entries = flat_dir_entries();
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].path, std::path::PathBuf::from("src/flat"));
    }
}
