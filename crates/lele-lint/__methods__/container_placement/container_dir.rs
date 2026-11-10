use std::path::{Path, PathBuf};

pub fn container_dir(rel_path: &Path, folder: &str) -> PathBuf {
    let mut components = rel_path.components();
    let _file = components.next_back();
    match components.next() {
        Some(domain) => PathBuf::from(domain.as_os_str()).join(folder),
        None => PathBuf::from(folder),
    }
}

// no test_usage necessary
