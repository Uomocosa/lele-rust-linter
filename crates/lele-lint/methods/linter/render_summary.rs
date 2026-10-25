use std::collections::BTreeMap;
use std::collections::BTreeSet;

use crate::Linter;
use crate::RuleViolation;

pub fn render_summary(violations: &[RuleViolation], color: bool) -> String {
    if violations.is_empty() {
        return String::new();
    }
    let mut per_code: BTreeMap<&str, usize> = BTreeMap::new();
    for violation in violations {
        let count = per_code.entry(violation.code).or_default();
        *count = count.saturating_add(1);
    }
    let files = violations
        .iter()
        .map(|violation| violation.file.as_path())
        .collect::<BTreeSet<_>>()
        .len();
    let mut ranked = per_code.into_iter().collect::<Vec<_>>();
    ranked.sort_by_key(|(code, count)| (std::cmp::Reverse(*count), *code));
    let counts = ranked
        .chunks(6)
        .map(|row| {
            row.iter()
                .map(|(code, count)| format!("{code} ×{count:<3}"))
                .collect::<Vec<_>>()
                .join(" ")
                .trim_end()
                .to_owned()
        })
        .collect::<Vec<_>>()
        .join("\n  ");
    let plural = |count: usize, word: &str| {
        if count == 1 {
            format!("{count} {word}")
        } else {
            format!("{count} {word}s")
        }
    };
    format!(
        "{}{}\n  {counts}\n",
        Linter::paint(color, "1;31", "error"),
        Linter::paint(
            color,
            "1",
            &format!(
                ": {} in {}",
                plural(violations.len(), "violation"),
                plural(files, "file")
            )
        )
    )
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::render_summary;
    use crate::RuleViolation;

    #[test]
    fn test_usage() {
        let violations = vec![
            RuleViolation::from_file(PathBuf::from("a.rs"), "LELE002", String::from("m")),
            RuleViolation::from_file(PathBuf::from("a.rs"), "LELE001", String::from("m")),
            RuleViolation::from_file(PathBuf::from("b.rs"), "LELE001", String::from("m")),
        ];
        assert_eq!(
            render_summary(&violations, false),
            "error: 3 violations in 2 files\n  LELE001 ×2   LELE002 ×1\n"
        );
        assert_eq!(render_summary(&[], false), "");
    }
}
