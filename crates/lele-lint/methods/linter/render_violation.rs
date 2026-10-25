use std::fmt::Write as _;

use crate::Linter;
use crate::Location;
use crate::RuleViolation;

pub fn render_violation(violation: &RuleViolation, source: Option<&str>, color: bool) -> String {
    let (message, help) = violation
        .message
        .split_once(" — ")
        .map_or((violation.message.as_str(), None), |(m, h)| (m, Some(h)));
    let site = match violation.location {
        Location::File => None,
        Location::Site { line, col } => Some((line, col)),
    };
    let gutter = " ".repeat(site.map_or(1, |(line, _)| line.to_string().len()));
    let bar = Linter::paint(color, "1;34", "|");
    let mut out = String::new();
    let _ = writeln!(
        out,
        "{}{}",
        Linter::paint(color, "1;31", &format!("error[{}]", violation.code)),
        Linter::paint(color, "1", &format!(": {message}"))
    );
    let position = site.map_or_else(String::new, |(line, col)| {
        format!(":{line}:{}", col.saturating_add(1))
    });
    let _ = writeln!(
        out,
        "{gutter}{} {}{position}",
        Linter::paint(color, "1;34", "-->"),
        violation.file.display()
    );
    let snippet = site.and_then(|(line, col)| {
        source
            .and_then(|s| s.lines().nth(line.saturating_sub(1)))
            .map(|text| (line, col, text))
    });
    if let Some((line, col, text)) = snippet {
        let width = text
            .chars()
            .skip(col)
            .take_while(|c| c.is_alphanumeric() || *c == '_')
            .count()
            .max(1);
        let _ = writeln!(out, "{gutter} {bar}");
        let _ = writeln!(
            out,
            "{} {bar} {text}",
            Linter::paint(color, "1;34", &line.to_string())
        );
        let _ = writeln!(
            out,
            "{gutter} {bar} {}{}",
            " ".repeat(col),
            Linter::paint(color, "1;31", &"^".repeat(width))
        );
    }
    if let Some(help) = help {
        let _ = writeln!(
            out,
            "{gutter} = {}: {help}",
            Linter::paint(color, "1;36", "help")
        );
    }
    let binary = if violation.code.starts_with("LELEBEVY") {
        "lele-bevy-lint"
    } else {
        "lele-lint"
    };
    let _ = writeln!(
        out,
        "{gutter} = {}: `{binary} --explain {}` shows why, with good/bad examples\n",
        Linter::paint(color, "1", "note"),
        violation.code
    );
    out
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::render_violation;
    use crate::RuleViolation;

    #[test]
    fn test_usage() {
        let violation = RuleViolation::from_site(
            PathBuf::from("a.rs"),
            2,
            4,
            "LELE001",
            String::from("too many items — move `spawn` to `spawn.rs`"),
        );
        let out = render_violation(&violation, Some("mod x;\npub fn spawn() {}\n"), false);
        assert!(out.starts_with("error[LELE001]: too many items\n"));
        assert!(out.contains("--> a.rs:2:5\n"));
        assert!(out.contains("2 | pub fn spawn() {}\n"));
        assert!(out.contains("  |     ^^^^^^^^^^^^^^^^\n") || out.contains("|     ^^"));
        assert!(out.contains("= help: move `spawn` to `spawn.rs`\n"));
        assert!(out.contains("`lele-lint --explain LELE001`"));
    }
}
