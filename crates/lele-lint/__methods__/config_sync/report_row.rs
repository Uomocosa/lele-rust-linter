use crate::ConfigReportRow;
use crate::RelaxedRule;
use crate::RuleSnapshot;

pub fn report_row(entry: &RuleSnapshot, relaxed: &[RelaxedRule]) -> ConfigReportRow {
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
    ConfigReportRow {
        code: entry.code.clone(),
        name: entry.name.clone(),
        namespace: entry.namespace.clone(),
        locked: if entry.is_locked {
            String::from("locked")
        } else {
            String::new()
        },
        level: String::from(level),
        exempts: exempt_reasons.len().to_string(),
        reason: String::from(reason),
    }
}

#[cfg(test)]
mod tests {
    use super::report_row;
    use crate::RelaxedRule;
    use crate::RuleSnapshot;

    #[test]
    fn test_usage() {
        let entry = RuleSnapshot {
            name: String::from("domain_import"),
            code: String::from("LELE016"),
            namespace: String::from("lint"),
            is_locked: false,
        };
        let row = report_row(&entry, &[]);
        assert_eq!(row.code, "LELE016");
        assert_eq!(row.locked, "");
        assert_eq!(row.level, "deny");
        assert_eq!(row.exempts, "0");
        assert_eq!(row.reason, "");
        let relaxed = vec![RelaxedRule::Off {
            name: String::from("domain_import"),
            code: String::from("LELE016"),
            reason: String::from("legacy"),
        }];
        let off = report_row(&entry, &relaxed);
        assert_eq!(off.level, "off");
        assert_eq!(off.reason, "legacy");
    }
}
