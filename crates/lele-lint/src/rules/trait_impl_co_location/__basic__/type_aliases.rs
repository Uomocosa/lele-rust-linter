use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::path::PathBuf;

pub(crate) type DefinitionMap = BTreeMap<String, BTreeSet<PathBuf>>;

// no test_usage necessary
