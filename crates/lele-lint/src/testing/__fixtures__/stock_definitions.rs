use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::path::PathBuf;

use crate::testing;

pub(crate) fn stock_definitions(
    type_definitions: &mut BTreeMap<String, BTreeSet<PathBuf>>,
    trait_definitions: &mut BTreeMap<String, BTreeSet<PathBuf>>,
) {
    testing::define_type(type_definitions, "stock/unix_time.rs", "UnixTime");
    testing::define_type(type_definitions, "stock/wrapper.rs", "Wrapper");
    testing::define_type(type_definitions, "stock/__basic__/structs.rs", "BasicOnly");
    testing::define_type(trait_definitions, "stock/__traits__/clock.rs", "Clock");
}

#[cfg(test)]
mod tests {
    use super::stock_definitions;
    use std::collections::BTreeMap;

    #[test]
    fn test_usage() {
        let mut types = BTreeMap::new();
        let mut traits = BTreeMap::new();
        stock_definitions(&mut types, &mut traits);
        assert!(traits.contains_key("Clock"));
    }
}
