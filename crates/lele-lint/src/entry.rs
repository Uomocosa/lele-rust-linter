use std::collections::HashMap;
use std::path::Path;
use std::path::PathBuf;

use atomic_delegate_macros::atomic_delegates;

use crate::EntryKind;
use crate::Error;
use crate::ParseFailure;

pub struct Entry {
    pub relative_path: PathBuf,
    pub absolute_path: PathBuf,
    pub kind: EntryKind,
}

#[atomic_delegates]
impl Entry {
    pub fn walk(walk_root: &Path, strip_base: &Path) -> Result<Vec<Self>, Error> {}
    pub fn parse_all(entries: &[Self]) -> (HashMap<PathBuf, syn::File>, Vec<ParseFailure>) {}
}

#[cfg(test)]
mod tests {
    use crate::Entry;

    #[test]
    fn test_usage() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("a.rs"), "pub fn a() {}\n").unwrap();
        let entries = Entry::walk(dir.path(), dir.path()).unwrap();
        let (parsed, failures) = Entry::parse_all(&entries);
        assert_eq!(parsed.len(), 1);
        assert!(failures.is_empty());
    }
}
