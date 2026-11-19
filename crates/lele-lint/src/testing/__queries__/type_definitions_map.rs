use std::collections::BTreeMap;

use crate::methods;
use crate::testing;
use crate::Project;

pub(crate) fn type_definitions_map(
    files: &[(&str, &str)],
) -> BTreeMap<String, Vec<std::path::PathBuf>> {
    let project: Project = testing::parsed_project(Project::default(), files);
    methods::trait_impl_co_location::type_definitions(&project)
        .into_iter()
        .map(|(name, paths)| (name, paths.into_iter().collect()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::type_definitions_map;

    #[test]
    fn test_usage() {
        let defined = type_definitions_map(&[("stock/unix_time.rs", "pub struct UnixTime;")]);
        assert!(defined.contains_key("UnixTime"));
    }
}
