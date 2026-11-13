use crate::Identifier;
use std::collections::HashMap;
use std::collections::HashSet;
use std::path::PathBuf;

pub fn find_parent_type(
    stem: &str,
    known_stems: &HashSet<String>,
    type_dirs: &HashMap<String, Vec<PathBuf>>,
) -> Option<Vec<PathBuf>> {
    Identifier(stem)
        .longest_type_prefix(known_stems)
        .and_then(|prefix| type_dirs.get(prefix).cloned())
}

#[cfg(test)]
mod tests {
    use crate::rules;
    use std::collections::HashMap;
    use std::collections::HashSet;
    use std::path::PathBuf;

    #[test]
    fn test_usage() {
        let known: HashSet<String> = HashSet::from(["thing".to_string()]);
        let dirs: HashMap<String, Vec<PathBuf>> =
            HashMap::from([("thing".to_string(), vec![PathBuf::from("src/thing")])]);
        assert_eq!(
            rules::method_file_co_location::MethodFileCoLocation::find_parent_type(
                "thing_run",
                &known,
                &dirs
            ),
            Some(vec![PathBuf::from("src/thing")])
        );
        assert!(
            rules::method_file_co_location::MethodFileCoLocation::find_parent_type(
                "other_run",
                &known,
                &dirs
            )
            .is_none()
        );
    }
}
