use crate::DunderPath;
use std::ffi::OsStr;

pub fn default_path(dunder: &DunderPath) -> String {
    let name = dunder
        .file_name()
        .and_then(OsStr::to_str)
        .unwrap_or_default();
    if dunder.is_dunder_dir() {
        format!("{name}/mod.rs")
    } else {
        name.to_string()
    }
}

// no test_usage necessary
