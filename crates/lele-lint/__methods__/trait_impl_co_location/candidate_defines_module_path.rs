use std::path::Path;

pub fn candidate_defines_module_path(candidate: &Path, module_path: &[String]) -> bool {
    let mut parent_components = candidate.components().collect::<Vec<_>>();
    parent_components.pop();
    let mut stripped_parents = Vec::new();
    let mut mounted_parents = Vec::new();
    for component in &parent_components {
        let name = component.as_os_str().to_string_lossy().into_owned();
        let module_name = name
            .strip_prefix("__")
            .and_then(|rest| rest.strip_suffix("__"))
            .map_or_else(|| name.clone(), str::to_string);
        let is_dunder_component = module_name != name;
        if !is_dunder_component {
            stripped_parents.push(name.clone());
        }
        mounted_parents.push(module_name);
    }
    if module_path.is_empty() {
        return parent_components.is_empty();
    }
    let stem = candidate
        .file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or("")
        .to_string();
    let mut stripped_chain = stripped_parents.clone();
    stripped_chain.push(stem.clone());
    let mut mounted_chain = mounted_parents.clone();
    mounted_chain.push(stem);
    let chains = [
        stripped_parents,
        mounted_parents,
        stripped_chain,
        mounted_chain,
    ];
    chains.iter().any(|chain| chain.as_slice() == module_path)
}

#[cfg(test)]
mod tests {
    use super::candidate_defines_module_path;

    fn module(names: &[&str]) -> Vec<String> {
        names.iter().map(|name| name.to_string()).collect()
    }

    #[test]
    fn test_usage() {
        assert!(candidate_defines_module_path(
            std::path::Path::new("discovery/config.rs"),
            &module(&["discovery"]),
        ));
        assert!(!candidate_defines_module_path(
            std::path::Path::new("discovery/config.rs"),
            &module(&["direct_dial"]),
        ));
    }

    #[test]
    fn test_usage_dunder_dirs_are_invisible_to_code_paths() {
        assert!(candidate_defines_module_path(
            std::path::Path::new("direct_dial/__basic__/resources.rs"),
            &module(&["direct_dial"]),
        ));
    }

    #[test]
    fn test_usage_dunder_mount_module_names_match_candidates() {
        assert!(candidate_defines_module_path(
            std::path::Path::new("stock/__traits__/clock.rs"),
            &module(&["stock", "traits"]),
        ));
        assert!(candidate_defines_module_path(
            std::path::Path::new("stock/__traits__/clock.rs"),
            &module(&["stock", "traits", "clock"]),
        ));
        assert!(candidate_defines_module_path(
            std::path::Path::new("stock/__basic__/structs.rs"),
            &module(&["stock", "basic"]),
        ));
        assert!(!candidate_defines_module_path(
            std::path::Path::new("stock/__traits__/clock.rs"),
            &module(&["other", "traits"]),
        ));
    }

    #[test]
    fn test_usage_dunder_stripped_matching_is_unchanged() {
        assert!(candidate_defines_module_path(
            std::path::Path::new("direct_dial/__basic__/resources.rs"),
            &module(&["direct_dial"]),
        ));
        assert!(candidate_defines_module_path(
            std::path::Path::new("a/__traits__/foo.rs"),
            &module(&["a"]),
        ));
        assert!(!candidate_defines_module_path(
            std::path::Path::new("a/__traits__/foo.rs"),
            &module(&["traits"]),
        ));
    }

    #[test]
    fn test_usage_nested_modules_do_not_match_root_modules() {
        assert!(!candidate_defines_module_path(
            std::path::Path::new("a/b/config.rs"),
            &module(&["b"]),
        ));
        assert!(candidate_defines_module_path(
            std::path::Path::new("a/b/config.rs"),
            &module(&["a", "b"]),
        ));
    }

    #[test]
    fn test_usage_qualified_files_match_their_module_path() {
        assert!(candidate_defines_module_path(
            std::path::Path::new("a/b.rs"),
            &module(&["a", "b"]),
        ));
        assert!(candidate_defines_module_path(
            std::path::Path::new("discovery/config.rs"),
            &module(&["discovery", "config"]),
        ));
        assert!(!candidate_defines_module_path(
            std::path::Path::new("discovery/config.rs"),
            &module(&["discovery", "other"]),
        ));
    }

    #[test]
    fn test_usage_crate_root_types_match_crate_root_files() {
        assert!(candidate_defines_module_path(
            std::path::Path::new("config.rs"),
            &[],
        ));
        assert!(!candidate_defines_module_path(
            std::path::Path::new("a/config.rs"),
            &[],
        ));
    }
}

// no test_usage necessary
