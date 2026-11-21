use std::fmt::Write as _;

use crate::RelaxedRule;
use crate::RuleSnapshot;

pub fn render_config_report(snapshot: &[RuleSnapshot], relaxed: &[RelaxedRule]) -> String {
    let mut out = String::from("CODE  NAME  NAMESPACE  LOCKED  LEVEL  EXEMPTS  REASON\n");
    for entry in snapshot {
        let off_reason = relaxed.iter().find_map(|rule| match rule {
            RelaxedRule::Off { name, reason, .. } if name == &entry.name => Some(reason),
            _ => None,
        });
        let exempt_reasons: Vec<&str> = relaxed
            .iter()
            .filter_map(|rule| match rule {
                RelaxedRule::Exempt { name, reason, .. } if name == &entry.name => {
                    Some(reason.as_str())
                }
                _ => None,
            })
            .collect();
        let level = if off_reason.is_some() { "off" } else { "deny" };
        let reason = off_reason
            .map(String::as_str)
            .or(exempt_reasons.first().copied())
            .unwrap_or("");
        let locked = if entry.is_locked { "locked" } else { "" };
        let _ = write!(
            out,
            "{}  {}  {}  {locked}  {level}  {}",
            entry.code,
            entry.name,
            entry.namespace,
            exempt_reasons.len()
        );
        if reason.is_empty() {
            out.push('\n');
        } else {
            let _ = writeln!(out, "  {reason}");
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::render_config_report;
    use crate::RelaxedRule;
    use crate::RuleSnapshot;

    #[test]
    fn test_usage() {
        let snapshot = vec![
            RuleSnapshot {
                name: String::from("domain_import"),
                code: String::from("LELE016"),
                namespace: String::from("lint"),
                is_locked: false,
            },
            RuleSnapshot {
                name: String::from("no_comments"),
                code: String::from("LELECORE015"),
                namespace: String::from("lint"),
                is_locked: true,
            },
        ];
        let report = render_config_report(&snapshot, &[]);
        assert!(report.starts_with("CODE  NAME  NAMESPACE  LOCKED  LEVEL  EXEMPTS  REASON\n"));
        assert!(report.contains("LELE016  domain_import  lint    deny  0\n"));
        assert!(report.contains("LELECORE015  no_comments  lint  locked  deny  0\n"));
    }

    #[test]
    fn off_rule_shows_off_and_its_reason() {
        let snapshot = vec![RuleSnapshot {
            name: String::from("domain_import"),
            code: String::from("LELE016"),
            namespace: String::from("lint"),
            is_locked: false,
        }];
        let relaxed = vec![RelaxedRule::Off {
            name: String::from("domain_import"),
            code: String::from("LELE016"),
            reason: String::from("legacy"),
        }];
        let report = render_config_report(&snapshot, &relaxed);
        assert!(report.contains("LELE016  domain_import  lint    off  0  legacy\n"));
    }

    #[test]
    fn exempt_rule_shows_count_and_first_reason() {
        let snapshot = vec![RuleSnapshot {
            name: String::from("no_comments"),
            code: String::from("LELECORE015"),
            namespace: String::from("lint"),
            is_locked: true,
        }];
        let relaxed = vec![
            RelaxedRule::Exempt {
                name: String::from("no_comments"),
                code: String::from("LELECORE015"),
                path: std::path::PathBuf::from("src/legacy"),
                reason: String::from("mirror"),
                replaced_by: None,
            },
            RelaxedRule::Exempt {
                name: String::from("no_comments"),
                code: String::from("LELECORE015"),
                path: std::path::PathBuf::from("src/other"),
                reason: String::from("vendored"),
                replaced_by: None,
            },
        ];
        let report = render_config_report(&snapshot, &relaxed);
        assert!(report.contains("LELECORE015  no_comments  lint  locked  deny  2  mirror\n"));
    }
}
