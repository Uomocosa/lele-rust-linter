use std::collections::HashSet;

pub(crate) fn string_set(items: &[&str]) -> HashSet<String> {
    items.iter().map(|item| (*item).to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::string_set;

    #[test]
    fn test_usage() {
        assert!(string_set(&["a"]).contains("a"));
    }
}
