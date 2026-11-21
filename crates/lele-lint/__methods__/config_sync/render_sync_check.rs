use std::fmt::Write as _;

use crate::SyncOutcome;

pub fn render_sync_check(outcome: &SyncOutcome, binary: &str) -> String {
    if outcome.missing.is_empty() && outcome.refreshed.is_empty() && !outcome.is_header_added {
        return String::new();
    }
    let mut out = format!("lele.toml is out of sync; run {binary} --sync-config\n");
    if outcome.is_header_added {
        let _ = writeln!(out, "  missing header");
    }
    for name in &outcome.missing {
        let _ = writeln!(out, "  missing table: {name}");
    }
    for name in &outcome.refreshed {
        let _ = writeln!(out, "  refresh comment: {name}");
    }
    out
}

#[cfg(test)]
mod tests {
    use super::render_sync_check;
    use crate::SyncOutcome;

    #[test]
    fn test_usage() {
        let synced = SyncOutcome::default();
        assert_eq!(render_sync_check(&synced, "lele-lint"), "");
        let stale = SyncOutcome {
            missing: vec![String::from("lele.rules.lint.domain_import")],
            refreshed: vec![String::from("lele.rules.lint.no_comments")],
            is_header_added: true,
        };
        let report = render_sync_check(&stale, "lele-lint");
        assert!(report.contains("run lele-lint --sync-config"));
        assert!(report.contains("missing table: lele.rules.lint.domain_import"));
        assert!(report.contains("refresh comment: lele.rules.lint.no_comments"));
        assert!(report.contains("missing header"));
    }
}
