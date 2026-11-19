use crate::common;
use std::path::Path;

pub fn impl_file_module_path(rel_path: &Path) -> Vec<String> {
    let segments = common::SourcePath(rel_path).module_path();
    segments
        .iter()
        .map(|segment| {
            segment
                .strip_prefix("__")
                .and_then(|rest| rest.strip_suffix("__"))
                .map_or_else(|| segment.clone(), str::to_string)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::impl_file_module_path;
    use crate::testing;
    use std::path::Path;

    #[test]
    fn test_usage() {
        assert_eq!(
            impl_file_module_path(Path::new("dial/setup.rs")),
            testing::strings(&["dial", "setup"])
        );
    }

    #[test]
    fn test_usage_maps_dunder_folders_to_code_module_names() {
        assert_eq!(
            impl_file_module_path(Path::new("stock/__traits__/clock.rs")),
            testing::strings(&["stock", "traits", "clock"])
        );
        assert_eq!(
            impl_file_module_path(Path::new("stock/__basic__/structs.rs")),
            testing::strings(&["stock", "basic", "structs"])
        );
    }

    #[test]
    fn test_usage_index_files_drop_their_stem() {
        assert_eq!(
            impl_file_module_path(Path::new("stock/mod.rs")),
            testing::strings(&["stock"])
        );
        assert_eq!(
            impl_file_module_path(Path::new("lib.rs")),
            testing::strings(&[])
        );
    }
}
