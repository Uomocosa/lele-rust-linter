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
    use crate::testing;

    #[test]
    fn test_usage() {
        let prefix = testing::strings(&["greeting"]);
        assert_eq!(
            reexport_suggestion(
                &prefix,
                &testing::strings(&["traits", "Greetable"]),
                false,
                "traits"
            ),
            "greeting::traits::Greetable"
        );
    }

    #[test]
    fn test_usage_crate_paths_drop_the_prefix() {
        assert_eq!(
            reexport_suggestion(
                &[],
                &testing::strings(&["crate", "stock", "traits", "Clock"]),
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
                &testing::strings(&["other_crate", "domain", "traits", "X"]),
                false,
                "traits"
            ),
            "other_crate::domain::traits::X"
        );
        assert_eq!(
            reexport_suggestion(
                &[],
                &testing::strings(&["stock", "traits", "Clock"]),
                false,
                "traits"
            ),
            "stock::traits::Clock"
        );
    }

    #[test]
    fn test_usage_self_paths_get_the_prefix() {
        let prefix = testing::strings(&["stock"]);
        assert_eq!(
            reexport_suggestion(
                &prefix,
                &testing::strings(&["self", "traits", "Clock"]),
                false,
                "traits"
            ),
            "stock::traits::Clock"
        );
    }

    #[test]
    fn test_usage_globs_get_a_star_suffix() {
        let prefix = testing::strings(&["greeting"]);
        assert_eq!(
            reexport_suggestion(&prefix, &testing::strings(&["traits"]), true, "traits"),
            "greeting::traits::*"
        );
    }
}
