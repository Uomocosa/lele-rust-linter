use crate::traits::ToLeleSnakeCase;

pub struct DelegateFile;

impl DelegateFile {
    #[must_use]
    pub fn stem(trait_ident: Option<&str>, method: &str, generic_suffix: &str) -> String {
        let method_stem = match trait_ident.map(ToLeleSnakeCase::to_snake_case) {
            Some(prefix) if !Self::names_method(&prefix, method) => format!("{prefix}_{method}"),
            _ => method.to_owned(),
        };
        format!("{method_stem}{generic_suffix}")
    }
    #[must_use]
    pub(crate) fn names_method(prefix: &str, method: &str) -> bool {
        method
            .strip_prefix(prefix)
            .is_some_and(|rest| rest.is_empty() || rest.starts_with('_'))
    }
}

#[cfg(test)]
mod tests {
    use super::DelegateFile;

    #[test]
    fn test_usage() {
        assert_eq!(
            DelegateFile::stem(Some("Display"), "fmt", ""),
            "display_fmt"
        );
        assert_eq!(
            DelegateFile::stem(Some("From"), "from", "_celsius"),
            "from_celsius"
        );
        assert_eq!(
            DelegateFile::stem(Some("TryFrom"), "try_from", "_path"),
            "try_from_path"
        );
        assert_eq!(
            DelegateFile::stem(Some("Visit"), "visit_item_mod", ""),
            "visit_item_mod"
        );
        assert_eq!(DelegateFile::stem(Some("Rule"), "check", ""), "rule_check");
        assert_eq!(
            DelegateFile::stem(Some("Default"), "default", ""),
            "default"
        );
        assert_eq!(DelegateFile::stem(None, "now", ""), "now");
    }

    #[test]
    fn test_usage_prefix_needs_word_boundary() {
        assert_eq!(DelegateFile::stem(Some("Rule"), "rules", ""), "rule_rules");
    }
}
