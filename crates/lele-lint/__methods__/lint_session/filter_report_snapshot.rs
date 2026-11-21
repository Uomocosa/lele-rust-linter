use crate::RuleSnapshot;

pub fn filter_report_snapshot(
    snapshot: &[RuleSnapshot],
    codes: Option<&[String]>,
) -> Result<Vec<RuleSnapshot>, Vec<String>> {
    let Some(codes) = codes else {
        return Ok(snapshot.to_vec());
    };
    let unknown: Vec<String> = codes
        .iter()
        .filter(|code| {
            !snapshot
                .iter()
                .any(|entry| entry.code.eq_ignore_ascii_case(code) || entry.name == code.as_str())
        })
        .cloned()
        .collect();
    if !unknown.is_empty() {
        return Err(unknown);
    }
    Ok(snapshot
        .iter()
        .filter(|entry| {
            codes
                .iter()
                .any(|code| entry.code.eq_ignore_ascii_case(code) || entry.name == code.as_str())
        })
        .cloned()
        .collect())
}

#[cfg(test)]
mod tests {
    use super::filter_report_snapshot;
    use crate::LintSession;
    use crate::Linter;

    #[test]
    fn test_usage() {
        let snapshot = LintSession::collect_snapshot(&Linter::default());
        let filtered =
            filter_report_snapshot(&snapshot, Some(&[String::from("no_comments")])).unwrap();
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].name, "no_comments");
    }

    #[test]
    fn missing_codes_keeps_every_entry() {
        let snapshot = LintSession::collect_snapshot(&Linter::default());
        let filtered = filter_report_snapshot(&snapshot, None).unwrap();
        assert_eq!(filtered.len(), snapshot.len());
    }

    #[test]
    fn unknown_code_is_rejected() {
        let snapshot = LintSession::collect_snapshot(&Linter::default());
        let unknown =
            filter_report_snapshot(&snapshot, Some(&[String::from("no_such_rule")])).unwrap_err();
        assert_eq!(unknown, vec![String::from("no_such_rule")]);
    }
}
