use std::collections::BTreeSet;
use std::path::PathBuf;

use crate::rules;

pub fn narrowed_candidates(files: &BTreeSet<PathBuf>, hints: &[Vec<String>]) -> Vec<PathBuf> {
    let candidates = files
        .iter()
        .filter(|file| {
            hints.iter().any(|hint| {
                rules::trait_impl_co_location::TraitImplCoLocation::candidate_defines_module_path(
                    file, hint,
                )
            })
        })
        .cloned();
    candidates.collect()
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::narrowed_candidates;
    use crate::testing;

    #[test]
    fn test_usage() {
        let narrowed = narrowed_candidates(
            &testing::path_sorted_set(&[
                "discovery/config.rs",
                "direct_dial/__basic__/resources.rs",
            ]),
            &testing::nested_strings(&[&["discovery"]]),
        );
        assert_eq!(narrowed, vec![PathBuf::from("discovery/config.rs")]);
    }

    #[test]
    fn test_usage_without_hints_nothing_is_narrowed() {
        assert_eq!(
            narrowed_candidates(
                &testing::path_sorted_set(&["discovery/config.rs"]),
                &testing::nested_strings(&[]),
            ),
            Vec::<PathBuf>::new()
        );
    }

    #[test]
    fn test_usage_every_hint_contributes_its_candidates() {
        let narrowed = narrowed_candidates(
            &testing::path_sorted_set(&["a/config.rs", "b/config.rs"]),
            &testing::nested_strings(&[&["a"], &["b"]]),
        );
        assert_eq!(
            narrowed,
            vec![PathBuf::from("a/config.rs"), PathBuf::from("b/config.rs")]
        );
    }
}

// no test_usage necessary
