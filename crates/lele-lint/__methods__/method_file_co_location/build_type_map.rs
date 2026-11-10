use crate::common;
use crate::Project;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub fn build_type_map(project: &Project) -> HashMap<String, Vec<PathBuf>> {
    let mut map = HashMap::<String, Vec<PathBuf>>::new();
    for (rel_path, file) in &project.parsed_files {
        let Some(stem) = rel_path.file_stem().and_then(|s| s.to_str()) else {
            continue;
        };
        if common::SourceFile(file).primary_type_name(stem).is_some() {
            let dir = rel_path.parent().unwrap_or(Path::new("")).to_path_buf();
            map.entry(stem.to_string()).or_default().push(dir);
        }
    }
    map
}

// no test_usage necessary
