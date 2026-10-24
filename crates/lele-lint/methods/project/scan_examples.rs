use crate::Entry;
use crate::Error;
use crate::ScannedDir;
use std::path::Path;

pub fn scan_examples(root: &Path) -> Result<ScannedDir, Error> {
    let dir = root.join("examples");
    if !dir.is_dir() {
        return Ok(ScannedDir::default());
    }
    let entries = Entry::walk(&dir, &dir)?;
    let (parsed_files, parse_failures) = Entry::parse_all(&entries);
    Ok(ScannedDir {
        entries,
        parsed_files,
        parse_failures,
    })
}

// no test_usage necessary
