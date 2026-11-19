pub(crate) fn sorted_hints(hints: &[Vec<String>]) -> Vec<Vec<String>> {
    let mut hints = hints.to_vec();
    hints.sort();
    hints
}

#[cfg(test)]
mod tests {
    use super::sorted_hints;

    #[test]
    fn test_usage() {
        assert_eq!(
            sorted_hints(&[vec!["b".to_string()], vec!["a".to_string()]]),
            vec![vec!["a".to_string()], vec!["b".to_string()]]
        );
    }
}
