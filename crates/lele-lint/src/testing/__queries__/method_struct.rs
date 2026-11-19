use std::collections::HashSet;

use crate::Identifier;

pub(crate) fn method_struct(file_name: &str, struct_names: &HashSet<String>) -> Option<String> {
    file_name
        .strip_suffix(".rs")
        .and_then(|stem| Identifier(stem).longest_type_prefix(struct_names))
        .map(str::to_string)
}

#[cfg(test)]
mod tests {
    use super::method_struct;
    use std::collections::HashSet;

    #[test]
    fn test_usage() {
        let names = HashSet::from(["player".to_string()]);
        assert_eq!(
            method_struct("player_new.rs", &names),
            Some("player".to_string())
        );
    }
}
