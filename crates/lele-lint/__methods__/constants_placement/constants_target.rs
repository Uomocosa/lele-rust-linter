use crate::Project;
use std::path::{Path, PathBuf};

pub fn constants_target(project: &Project, dir: &Path) -> PathBuf {
    if let Some(folder) = project.dunder_dir() {
        return dir.join(folder).join("constants.rs");
    }
    if dir.as_os_str().is_empty() {
        PathBuf::from("constants.rs")
    } else {
        dir.join("constants.rs")
    }
}

// no test_usage necessary
