use crate::Visibility;
use std::collections::HashMap;
use std::path::PathBuf;

pub fn collect_defined_consts(
    parsed_files: &HashMap<PathBuf, syn::File>,
) -> HashMap<String, PathBuf> {
    let mut out = HashMap::new();
    for (rel_path, file) in parsed_files {
        for item in &file.items {
            let (name, exposed) = match item {
                syn::Item::Const(c) => (c.ident.to_string(), Visibility(&c.vis).is_exposed()),
                syn::Item::Static(s) => (s.ident.to_string(), Visibility(&s.vis).is_exposed()),
                _ => continue,
            };
            if exposed {
                out.insert(name, rel_path.clone());
            }
        }
    }
    out
}

// no test_usage necessary
