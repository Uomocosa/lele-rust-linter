pub fn first_sentence(summary: &str, max_chars: usize) -> String {
    let mut in_code = false;
    let mut end = summary.len();
    let mut rest = summary.char_indices().peekable();
    while let Some((index, ch)) = rest.next() {
        if ch == '`' {
            in_code = !in_code;
        } else if ch == '.' && !in_code && rest.peek().is_none_or(|(_, next)| next.is_whitespace())
        {
            end = index.saturating_add(1);
            break;
        }
    }
    let first = summary.get(..end).unwrap_or(summary).trim();
    if first.chars().count() <= max_chars {
        return first.to_owned();
    }
    let kept: String = first.chars().take(max_chars).collect();
    let cut = kept
        .rfind(' ')
        .and_then(|byte| kept.get(..byte))
        .unwrap_or(&kept)
        .trim_end()
        .to_owned();
    let cut = {
        let mut chars = cut.chars();
        let mut open_at: Option<usize> = None;
        for (index, ch) in chars.by_ref().enumerate() {
            if ch == '`' {
                open_at = if open_at.is_none() { Some(index) } else { None };
            }
        }
        match open_at {
            Some(index) => cut
                .chars()
                .take(index)
                .collect::<String>()
                .trim_end()
                .to_owned(),
            None => cut,
        }
    };
    format!("{cut}…")
}

#[cfg(test)]
mod tests {
    use super::first_sentence;

    use rstest::rstest;

    #[test]
    fn test_usage() {
        assert_eq!(
            first_sentence("No comments here. Because reasons.", 100),
            "No comments here."
        );
        assert_eq!(first_sentence("No terminator", 100), "No terminator");
        assert_eq!(first_sentence("  Padded. Rest.", 100), "Padded.");
    }

    #[rstest]
    #[case(
        "A `<type>_<method>.rs` file must sit in the same folder as `<type>.rs`. Files under `__bevy_systems__/` are exempt (a Bevy system file may start with a type's stem).",
        94,
        "A `<type>_<method>.rs` file must sit in the same folder as `<type>.rs`."
    )]
    #[case(
        "No comments in `src/` or `__methods__/`, except `///` doc comments in `constants.rs` and a final `// no test_usage necessary`.",
        94,
        "No comments in `src/` or `__methods__/`, except `///` doc comments in `constants.rs` and a…"
    )]
    #[case("No terminator here", 100, "No terminator here")]
    #[case("Exactly ten", 11, "Exactly ten")]
    #[case("xx `code words here` end. Rest.", 12, "xx…")]
    fn sentence_cases(#[case] summary: &str, #[case] max_chars: usize, #[case] expected: &str) {
        assert_eq!(first_sentence(summary, max_chars), expected);
    }

    #[test]
    fn long_sentence_is_cut_at_a_word_boundary() {
        let shortened = first_sentence("Ässert everything über alles. Rest.", 10);
        assert_eq!(shortened, "Ässert…");
    }
}
