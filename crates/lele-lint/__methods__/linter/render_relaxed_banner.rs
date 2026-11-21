use std::fmt::Write as _;

use crate::RelaxedRule;

pub fn render_relaxed_banner(relaxed: &[RelaxedRule]) -> String {
    if relaxed.is_empty() {
        return String::new();
    }
    let mut out = format!("lele.toml relaxes {} rule(s):\n", relaxed.len());
    for rule in relaxed {
        let _ = writeln!(
            out,
            "  off  {} ({})  reason: {}",
            rule.name, rule.code, rule.reason
        );
    }
    out
}

#[cfg(test)]
mod tests {
    use super::render_relaxed_banner;
    use crate::Linter;
    use crate::RelaxedRule;

    #[test]
    fn test_usage() {
        assert_eq!(render_relaxed_banner(&[]), "");
        let relaxed = vec![
            RelaxedRule {
                name: String::from("no_positional"),
                code: String::from("LELECORE008"),
                reason: String::from("legacy file"),
            },
            RelaxedRule {
                name: String::from("no_comments"),
                code: String::from("LELECORE015"),
                reason: String::from("docs"),
            },
        ];
        assert_eq!(
            Linter::render_relaxed_banner(&relaxed),
            "lele.toml relaxes 2 rule(s):\n  off  no_positional (LELECORE008)  reason: legacy file\n  off  no_comments (LELECORE015)  reason: docs\n"
        );
    }
}
