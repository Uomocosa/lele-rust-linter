use crate::Project;
use std::path::Path;

pub fn is_actually_method_file(file_name: &str, parent_dir: &str, project: &Project) -> bool {
    let rel_path = if parent_dir.is_empty() {
        Path::new(file_name).to_path_buf()
    } else {
        Path::new(parent_dir).join(file_name)
    };

    let parsed = match project.get_parsed(&rel_path) {
        Some(f) => f,
        None => return true,
    };

    let has_type_definition = parsed
        .items
        .iter()
        .any(|item| matches!(item, syn::Item::Struct(_) | syn::Item::Enum(_)));

    if has_type_definition {
        return false;
    }

    parsed
        .items
        .iter()
        .any(|item| matches!(item, syn::Item::Fn(_)))
}

// no test_usage necessary
