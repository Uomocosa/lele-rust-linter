use crate::rules;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use syn::visit::Visit;

pub fn collect_const_usages(
    parsed_files: &HashMap<PathBuf, syn::File>,
    defined: &HashSet<String>,
) -> HashMap<String, HashSet<PathBuf>> {
    let mut usages: HashMap<String, HashSet<PathBuf>> = HashMap::new();
    for (rel_path, file) in parsed_files {
        if rel_path.components().any(|c| c.as_os_str() == "testing") {
            continue;
        }
        let mut visitor = rules::constants_placement::ConstUseVisitor {
            defined,
            found: Vec::new(),
        };
        visitor.visit_file(file);
        for name in visitor.found {
            usages.entry(name).or_default().insert(rel_path.clone());
        }
    }
    usages
}

// no test_usage necessary
