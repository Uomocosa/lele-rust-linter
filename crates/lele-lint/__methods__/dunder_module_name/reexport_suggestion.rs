pub fn reexport_suggestion(
    prefix: &[String],
    segments: &[String],
    is_glob: bool,
    mount: &str,
) -> String {
    let head = segments.first().map(String::as_str);
    let without_crate = segments.get(1..).unwrap_or_default();
    let full: Vec<String> = if head == Some("crate") {
        without_crate.to_vec()
    } else if head == Some(mount) {
        prefix.iter().chain(segments.iter()).cloned().collect()
    } else if head == Some("self") {
        prefix.iter().chain(without_crate.iter()).cloned().collect()
    } else {
        segments.to_vec()
    };
    let mut suggestion = full.join("::");
    if is_glob {
        suggestion.push_str("::*");
    }
    suggestion
}

#[cfg(test)]
mod tests {
    use super::reexport_suggestion;

    fn names(segments: &[&str]) -> Vec<String> {
        segments.iter().map(|segment| segment.to_string()).collect()
    }

    #[test]
    fn test_usage() {
        let prefix = names(&["greeting"]);
        assert_eq!(
            reexport_suggestion(&prefix, &names(&["traits", "Greetable"]), false, "traits"),
            "greeting::traits::Greetable"
        );
    }

    #[test]
    fn test_usage_crate_paths_drop_the_prefix() {
        assert_eq!(
            reexport_suggestion(
                &[],
                &names(&["crate", "stock", "traits", "Clock"]),
                false,
                "traits"
            ),
            "stock::traits::Clock"
        );
    }

    #[test]
    fn test_usage_qualified_paths_pass_through() {
        assert_eq!(
            reexport_suggestion(
                &[],
                &names(&["other_crate", "domain", "traits", "X"]),
                false,
                "traits"
            ),
            "other_crate::domain::traits::X"
        );
        assert_eq!(
            reexport_suggestion(&[], &names(&["stock", "traits", "Clock"]), false, "traits"),
            "stock::traits::Clock"
        );
    }

    #[test]
    fn test_usage_self_paths_get_the_prefix() {
        let prefix = names(&["stock"]);
        assert_eq!(
            reexport_suggestion(
                &prefix,
                &names(&["self", "traits", "Clock"]),
                false,
                "traits"
            ),
            "stock::traits::Clock"
        );
    }

    #[test]
    fn test_usage_globs_get_a_star_suffix() {
        let prefix = names(&["greeting"]);
        assert_eq!(
            reexport_suggestion(&prefix, &names(&["traits"]), true, "traits"),
            "greeting::traits::*"
        );
    }
}
