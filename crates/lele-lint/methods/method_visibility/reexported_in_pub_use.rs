use crate::ModuleInfoMap;

pub fn reexported_in_pub_use(
    module_info: &ModuleInfoMap,
    parent_dir: &str,
    mod_name: &str,
) -> Option<std::path::PathBuf> {
    let mod_rs_path = if parent_dir.is_empty() {
        std::path::PathBuf::from("mod.rs")
    } else {
        std::path::PathBuf::from(parent_dir).join("mod.rs")
    };

    let info = module_info.get(&mod_rs_path)?;

    for reexport in &info.reexports {
        if reexport.segments.len() == 1 && reexport.segments.first().is_some_and(|s| s == mod_name)
        {
            return Some(info.rel_path.clone());
        }
        if reexport.segments.len() == 2 && reexport.segments.get(1).is_some_and(|s| s == mod_name) {
            return Some(info.rel_path.clone());
        }
    }
    None
}

// no test_usage necessary
