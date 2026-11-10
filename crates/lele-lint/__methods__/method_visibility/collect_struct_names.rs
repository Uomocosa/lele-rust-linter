use crate::common;
use crate::Project;
use std::collections::HashSet;
use std::path::Path;

pub fn collect_struct_names(
    file_names: &[String],
    parent_dir: &str,
    project: &Project,
) -> HashSet<String> {
    let mut names = HashSet::new();
    for f in file_names {
        let Some(stem) = f.strip_suffix(".rs") else {
            continue;
        };
        let rel_path = if parent_dir.is_empty() {
            Path::new(f).to_path_buf()
        } else {
            Path::new(parent_dir).join(f)
        };
        let Some(parsed) = project.get_parsed(&rel_path) else {
            continue;
        };
        if common::SourceFile(parsed).primary_type_name(stem).is_some() {
            names.insert(stem.to_string());
        }
    }
    names
}

// no test_usage necessary
