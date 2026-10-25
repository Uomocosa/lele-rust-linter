use std::collections::BTreeMap;
use std::path::Path;

use crate::Linter;
use crate::Location;
use crate::RuleViolation;

pub fn render_report(violations: &[RuleViolation], color: bool) -> String {
    let mut by_file: BTreeMap<&Path, Vec<&RuleViolation>> = BTreeMap::new();
    for violation in violations {
        by_file
            .entry(violation.file.as_path())
            .or_default()
            .push(violation);
    }
    let mut out = String::new();
    for (file, mut group) in by_file {
        group.sort_by_key(|violation| match violation.location {
            Location::File => (0, 0, violation.code),
            Location::Site { line, col } => (line, col, violation.code),
        });
        if group.len() > 1 {
            let header = format!("── {} ({} errors) ──", file.display(), group.len());
            out.push_str(&Linter::paint(color, "1;33", &header));
            out.push_str("\n\n");
        }
        let source = std::fs::read_to_string(file).ok();
        for violation in group {
            out.push_str(&Linter::render_violation(
                violation,
                source.as_deref(),
                color,
            ));
        }
    }
    out.push_str(&Linter::render_summary(violations, color));
    out
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::render_report;
    use crate::RuleViolation;

    #[test]
    fn test_usage() {
        let violations = vec![
            RuleViolation::from_site(PathBuf::from("b.rs"), 9, 0, "LELE002", String::from("x")),
            RuleViolation::from_site(PathBuf::from("b.rs"), 3, 0, "LELE001", String::from("y")),
            RuleViolation::from_file(PathBuf::from("a.rs"), "LELE001", String::from("z")),
        ];
        let out = render_report(&violations, false);
        assert!(!out.contains("── a.rs"));
        let header = out.find("── b.rs (2 errors) ──").unwrap_or(usize::MAX);
        let first = out.find("error[LELE001]: y").unwrap_or(0);
        let second = out.find("error[LELE002]: x").unwrap_or(0);
        assert!(out.find("error[LELE001]: z") < Some(header));
        assert!(header < first && first < second);
        assert!(out.ends_with("error: 3 violations in 2 files\n  LELE001 ×2   LELE002 ×1\n"));
    }
}
