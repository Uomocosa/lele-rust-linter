use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::path::PathBuf;

use crate::testing;

pub(crate) fn same_named_config_definitions(
    type_definitions: &mut BTreeMap<String, BTreeSet<PathBuf>>,
) {
    testing::define_type(type_definitions, "discovery/config.rs", "Config");
    testing::define_type(
        type_definitions,
        "direct_dial/__basic__/resources.rs",
        "Config",
    );
}

#[cfg(test)]
mod tests {
    use super::same_named_config_definitions;
    use std::collections::BTreeMap;

    #[test]
    fn test_usage() {
        let mut types = BTreeMap::new();
        same_named_config_definitions(&mut types);
        assert_eq!(types["Config"].len(), 2);
    }
}
