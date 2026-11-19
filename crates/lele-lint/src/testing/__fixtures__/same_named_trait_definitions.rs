use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::path::PathBuf;

use crate::testing;

pub(crate) fn same_named_trait_definitions(
    trait_definitions: &mut BTreeMap<String, BTreeSet<PathBuf>>,
) {
    testing::define_type(trait_definitions, "a/__traits__/foo.rs", "Foo");
    testing::define_type(trait_definitions, "b/__traits__/foo.rs", "Foo");
}

#[cfg(test)]
mod tests {
    use super::same_named_trait_definitions;
    use std::collections::BTreeMap;

    #[test]
    fn test_usage() {
        let mut traits = BTreeMap::new();
        same_named_trait_definitions(&mut traits);
        assert_eq!(traits["Foo"].len(), 2);
    }
}
