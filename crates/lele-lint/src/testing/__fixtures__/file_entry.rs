use std::path::Path;
use std::path::PathBuf;

use crate::Entry;
use crate::EntryKind;

pub(crate) fn file_entry(dir: &Path, name: &str) -> Entry {
    Entry {
        relative_path: PathBuf::from(name),
        absolute_path: dir.join(name),
        kind: EntryKind::File,
    }
}

#[cfg(test)]
mod tests {
    use super::file_entry;
    use std::path::Path;

    #[test]
    fn test_usage() {
        let entry = file_entry(Path::new("/tmp"), "a.rs");
        assert_eq!(entry.relative_path.to_str(), Some("a.rs"));
    }
}
