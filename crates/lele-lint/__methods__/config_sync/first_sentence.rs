pub fn first_sentence(summary: &str, max_chars: usize) -> String {
    let first = summary
        .split_inclusive(['.', '!', '?'])
        .next()
        .unwrap_or(summary);
    let trimmed = first.trim();
    if trimmed.chars().count() <= max_chars {
        return trimmed.to_owned();
    }
    trimmed
        .chars()
        .take(max_chars)
        .collect::<String>()
        .trim_end()
        .to_owned()
}

#[cfg(test)]
mod tests {
    use super::first_sentence;

    #[test]
    fn test_usage() {
        assert_eq!(
            first_sentence("No comments here. Because reasons.", 100),
            "No comments here."
        );
        assert_eq!(first_sentence("No terminator", 100), "No terminator");
        assert_eq!(first_sentence("  Padded. Rest.", 100), "Padded.");
    }

    #[test]
    fn long_sentence_is_cut_at_a_char_boundary() {
        let summary = "Ässert everything über alles in every file. Rest.";
        let shortened = first_sentence(summary, 10);
        assert_eq!(shortened.chars().count(), 10);
        assert!(summary.starts_with(&shortened));
    }
}
