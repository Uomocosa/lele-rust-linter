pub fn line_of(source: &str, offset: usize) -> usize {
    source
        .get(..offset)
        .map_or(1, |prefix| prefix.matches('\n').count().saturating_add(1))
}

#[cfg(test)]
mod tests {
    use super::line_of;

    #[test]
    fn test_usage() {
        assert_eq!(line_of("a\nb\nc", 4), 3);
        assert_eq!(line_of("abc", 0), 1);
    }
}
