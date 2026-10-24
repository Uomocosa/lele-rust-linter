use crate::Project;
use std::path::Path;
use std::path::PathBuf;

pub fn base_prefix(project: &Project, base: &Path) -> PathBuf {
    let root = project
        .root
        .canonicalize()
        .unwrap_or_else(|_| project.root.clone());
    let base = base.canonicalize().unwrap_or_else(|_| base.to_path_buf());
    root.strip_prefix(&base)
        .unwrap_or(Path::new(""))
        .to_path_buf()
}

// no test_usage necessary
