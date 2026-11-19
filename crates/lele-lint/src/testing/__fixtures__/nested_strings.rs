pub(crate) fn nested_strings(items: &[&[&str]]) -> Vec<Vec<String>> {
    items
        .iter()
        .map(|item| item.iter().map(|segment| (*segment).to_string()).collect())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::nested_strings;

    #[test]
    fn test_usage() {
        assert_eq!(
            nested_strings(&[&["a", "b"]]),
            vec![vec!["a".to_string(), "b".to_string()]]
        );
    }
}
