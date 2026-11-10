use lele_snake_case::traits::ToLeleSnakeCase;
use std::path::Path;
use std::path::PathBuf;

pub fn trait_target_path(rel_path: &Path, trait_name: &str) -> PathBuf {
    let mut domain = rel_path
        .parent()
        .unwrap_or_else(|| Path::new(""))
        .to_path_buf();
    while domain
        .file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name.starts_with("__") && name.ends_with("__"))
    {
        domain.pop();
    }
    domain
        .join("__traits__")
        .join(format!("{}.rs", trait_name.to_snake_case()))
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::path::PathBuf;

    use super::trait_target_path;

    #[test]
    fn test_usage() {
        assert_eq!(
            trait_target_path(
                Path::new("room_sync/replicated_state.rs"),
                "ReplicatedState"
            ),
            PathBuf::from("room_sync/__traits__/replicated_state.rs")
        );
    }

    #[test]
    fn test_usage_crate_root_trait_targets_crate_root_dunder() {
        assert_eq!(
            trait_target_path(Path::new("rule.rs"), "Rule"),
            PathBuf::from("__traits__/rule.rs")
        );
    }

    #[test]
    fn test_usage_container_dunder_parent_is_popped() {
        assert_eq!(
            trait_target_path(Path::new("stock/__basic__/items.rs"), "Item"),
            PathBuf::from("stock/__traits__/item.rs")
        );
    }

    #[test]
    fn test_usage_traits_dir_parent_is_popped() {
        assert_eq!(
            trait_target_path(
                Path::new("stock/__traits__/replicated_state.rs"),
                "RepeatedState"
            ),
            PathBuf::from("stock/__traits__/repeated_state.rs")
        );
    }

    #[test]
    fn test_usage_nested_domain_owns_its_traits() {
        assert_eq!(
            trait_target_path(
                Path::new("stock/state/__traits__/replicated_state.rs"),
                "ReplicatedState"
            ),
            PathBuf::from("stock/state/__traits__/replicated_state.rs")
        );
    }
}

// no test_usage necessary
