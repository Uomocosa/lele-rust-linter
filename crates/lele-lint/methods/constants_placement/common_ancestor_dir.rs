use crate::rules;
use std::collections::HashSet;
use std::path::PathBuf;

pub fn common_ancestor_dir(users: &HashSet<PathBuf>) -> PathBuf {
    let mut iter = users.iter();
    let Some(first) = iter.next() else {
        return PathBuf::new();
    };
    let mut prefix: PathBuf = rules::constants_placement::ConstantsPlacement::parent_dir(first);
    for other in iter {
        let other_dir = rules::constants_placement::ConstantsPlacement::parent_dir(other);
        prefix = rules::constants_placement::ConstantsPlacement::common_prefix(&prefix, &other_dir);
    }
    prefix
}

// no test_usage necessary
