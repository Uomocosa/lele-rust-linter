use crate::ModuleInfoMap;

pub fn declared_as_pub_mod(
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

    for decl in &info.declarations {
        if decl.name == mod_name && decl.is_public {
            return Some(info.rel_path.clone());
        }
    }
    None
}

// no test_usage necessary
