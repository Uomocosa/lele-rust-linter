use crate::Entry;
use crate::EntryKind;
use std::path::Path;

pub fn group_entries_by_parent_dir(
    entries: &[Entry],
) -> std::collections::BTreeMap<String, Vec<String>> {
    let mut map: std::collections::BTreeMap<String, Vec<String>> =
        std::collections::BTreeMap::new();
    for entry in entries {
        if entry.kind != EntryKind::File {
            continue;
        }
        let parent = entry
            .relative_path
            .parent()
            .unwrap_or(Path::new(""))
            .to_string_lossy()
            .to_string();
        let Some(file_name) = entry.relative_path.file_name() else {
            continue;
        };
        let file_name = file_name.to_string_lossy().to_string();
        map.entry(parent).or_default().push(file_name);
    }
    map
}

// no test_usage necessary
