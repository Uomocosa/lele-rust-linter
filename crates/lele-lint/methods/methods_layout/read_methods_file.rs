use crate::EntryKind;
use crate::Project;
use std::path::Path;

pub fn read_methods_file(project: &Project, rel: &Path) -> Option<String> {
    let entry = project
        .methods_entries
        .iter()
        .find(|entry| entry.relative_path == rel && entry.kind == EntryKind::File)?;
    std::fs::read_to_string(&entry.absolute_path).ok()
}

// no test_usage necessary
