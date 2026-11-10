use std::path::Path;

pub fn candidate_defines_module_path(candidate: &Path, module_path: &[String]) -> bool {
    let mut parents = candidate.components().collect::<Vec<_>>();
    parents.pop();
    parents.retain(|component| {
        !component
            .as_os_str()
            .to_str()
            .is_some_and(|name| name.starts_with("__") && name.ends_with("__"))
    });
    if module_path.is_empty() {
        return parents.is_empty();
    }
    let parent_names = parents
        .iter()
        .map(|component| component.as_os_str().to_string_lossy().into_owned())
        .collect::<Vec<String>>();
    let mut chain_with_stem = parent_names.clone();
    chain_with_stem.push(
        candidate
            .file_stem()
            .and_then(|stem| stem.to_str())
            .unwrap_or("")
            .to_string(),
    );
    parent_names == module_path || chain_with_stem == module_path
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
