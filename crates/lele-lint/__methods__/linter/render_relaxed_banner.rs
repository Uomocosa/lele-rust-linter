use std::fmt::Write as _;

use crate::RelaxedRule;

pub fn render_relaxed_banner(relaxed: &[RelaxedRule]) -> String {
    if relaxed.is_empty() {
        return String::new();
    }
    let mut out = format!("lele.toml relaxes rules ({}):\n", relaxed.len());
    for rule in relaxed {
        match rule {
            RelaxedRule::Off { name, code, reason } => {
                let _ = writeln!(out, "  off  {name} ({code})  reason: {reason}");
            }
            RelaxedRule::Exempt {
                name,
                code,
                path,
                reason,
                replaced_by,
            } => {
                let _ = write!(out, "  exempt  {name} ({code})  {}  ", path.display());
                if let Some(replaced_by) = replaced_by {
                    let _ = write!(out, "replaced by {replaced_by}  ");
                }
                let _ = writeln!(out, "reason: {reason}");
            }
        }
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
            RelaxedRule::Off {
                name: String::from("no_positional"),
                code: String::from("LELECORE008"),
                reason: String::from("legacy file"),
            },
            RelaxedRule::Exempt {
                name: String::from("no_comments"),
                code: String::from("LELECORE015"),
                path: std::path::PathBuf::from("crates/rule-engine/__rules__"),
                reason: String::from("generated mirror"),
                replaced_by: None,
            },
            RelaxedRule::Exempt {
                name: String::from("domain_import"),
                code: String::from("LELE016"),
                path: std::path::PathBuf::from("src/legacy"),
                reason: String::from("vendored"),
                replaced_by: Some(String::from("test_usage")),
            },
        ];
        assert_eq!(
            Linter::render_relaxed_banner(&relaxed),
            "lele.toml relaxes rules (3):\n  off  no_positional (LELECORE008)  reason: legacy file\n  exempt  no_comments (LELECORE015)  crates/rule-engine/__rules__  reason: generated mirror\n  exempt  domain_import (LELE016)  src/legacy  replaced by test_usage  reason: vendored\n"
        );
    }
}
