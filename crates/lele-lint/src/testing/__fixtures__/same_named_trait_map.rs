use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::path::PathBuf;

use crate::testing;

pub(crate) fn same_named_trait_map() -> BTreeMap<String, BTreeSet<PathBuf>> {
    let mut trait_definitions = BTreeMap::new();
    testing::same_named_trait_definitions(&mut trait_definitions);
    trait_definitions
}

#[cfg(test)]
mod tests {
    use super::same_named_trait_map;

    #[test]
    fn test_usage() {
        assert_eq!(same_named_trait_map()["Foo"].len(), 2);
    }
}
