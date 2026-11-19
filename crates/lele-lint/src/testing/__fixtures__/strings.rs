pub(crate) fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|item| (*item).to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::strings;

    #[test]
    fn test_usage() {
        assert_eq!(strings(&["a", "b"]), vec!["a".to_string(), "b".to_string()]);
    }
}
