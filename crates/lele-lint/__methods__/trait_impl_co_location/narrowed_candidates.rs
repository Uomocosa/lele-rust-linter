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
    use std::collections::BTreeSet;
    use std::path::PathBuf;

    use super::narrowed_candidates;

    fn files(paths: &[&str]) -> BTreeSet<PathBuf> {
        paths.iter().map(PathBuf::from).collect()
    }

    fn hints(names: &[&[&str]]) -> Vec<Vec<String>> {
        names
            .iter()
            .map(|hint| hint.iter().map(|segment| segment.to_string()).collect())
            .collect()
    }

    #[test]
    fn test_usage() {
        let narrowed = narrowed_candidates(
            &files(&["discovery/config.rs", "direct_dial/__basic__/resources.rs"]),
            &hints(&[&["discovery"]]),
        );
        assert_eq!(narrowed, vec![PathBuf::from("discovery/config.rs")]);
    }

    #[test]
    fn test_usage_without_hints_nothing_is_narrowed() {
        assert_eq!(
            narrowed_candidates(&files(&["discovery/config.rs"]), &hints(&[]),),
            Vec::<PathBuf>::new()
        );
    }

    #[test]
    fn test_usage_every_hint_contributes_its_candidates() {
        let narrowed = narrowed_candidates(
            &files(&["a/config.rs", "b/config.rs"]),
            &hints(&[&["a"], &["b"]]),
        );
        assert_eq!(
            narrowed,
            vec![PathBuf::from("a/config.rs"), PathBuf::from("b/config.rs")]
        );
    }
}

// no test_usage necessary
