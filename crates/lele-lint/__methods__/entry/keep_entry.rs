use crate::Entry;

pub fn keep_entry(entry: &walkdir::DirEntry) -> bool {
    if !entry.file_type().is_dir() {
        return true;
    }
    !Entry::is_skipped_name(entry.file_name().to_str())
}

// no test_usage necessary
