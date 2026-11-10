use lele_snake_case::ToLeleSnakeCase;
use std::path::Path;
use std::path::PathBuf;

pub fn type_graduate_target(target: &Path, type_name: &str) -> PathBuf {
    let domain = target
        .parent()
        .and_then(Path::parent)
        .unwrap_or_else(|| Path::new(""));
    domain.join(format!("{}.rs", type_name.to_snake_case()))
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::path::PathBuf;

    use super::type_graduate_target;

    #[test]
    fn test_usage() {
        assert_eq!(
            type_graduate_target(Path::new("stock/__basic__/structs.rs"), "UnixTime"),
            PathBuf::from("stock/unix_time.rs")
        );
    }

    #[test]
    fn test_usage_uses_the_type_snake_name() {
        assert_eq!(
            type_graduate_target(Path::new("stock/__basic__/enums.rs"), "TimeSpan"),
            PathBuf::from("stock/time_span.rs")
        );
    }

    #[test]
    fn test_usage_nested_domain_graduates_in_place() {
        assert_eq!(
            type_graduate_target(Path::new("stock/state/__basic__/structs.rs"), "UnixTime"),
            PathBuf::from("stock/state/unix_time.rs")
        );
    }

    #[test]
    fn test_usage_crate_root_container_graduates_to_root() {
        assert_eq!(
            type_graduate_target(Path::new("__basic__/structs.rs"), "UnixTime"),
            PathBuf::from("unix_time.rs")
        );
    }
}

// no test_usage necessary
