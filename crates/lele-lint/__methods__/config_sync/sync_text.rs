use crate::traits::Rule;
use crate::ConfigSync;
use crate::Error;
use crate::SyncOutcome;

const HEADER: &str = "# CRITICAL (agents): never change this file unless the user asked for that change in this\n# conversation. A lint error is fixed in the code, not here. The [lele.rules.*] tables are\n# kept in sync by `lele-lint --sync-config` (and `lele-bevy-lint --sync-config`): it adds\n# new rules and refreshes the `# CODE · summary` lines; it never changes a level, reason\n# or exemption.\n";

pub fn sync_text(rules: &[Box<dyn Rule>], original: &str) -> Result<(String, SyncOutcome), Error> {
    let mut outcome = SyncOutcome::default();
    let (body, added) = match original.strip_prefix(HEADER) {
        Some(rest) => (rest, false),
        None => (original, true),
    };
    outcome.is_header_added = added;
    let mut doc: toml_edit::DocumentMut =
        body.parse().map_err(|source| Error::SyncParse { source })?;
    for rule in rules {
        ConfigSync::ensure_rule_table(&mut doc, rule.as_ref(), &mut outcome);
    }
    let synced = doc.to_string();
    let updated = if added {
        format!("{HEADER}\n{synced}")
    } else {
        format!("{HEADER}{synced}")
    };
    Ok((updated, outcome))
}

#[cfg(test)]
mod tests {
    use super::sync_text;
    use super::HEADER;
    use crate::Linter;

    #[test]
    fn test_usage() {
        let linter = Linter::default();
        let (updated, outcome) = sync_text(&linter.rules, "").unwrap();
        assert!(outcome.is_header_added);
        assert_ne!(outcome.missing, Vec::<String>::new());
        assert!(updated.contains("[lele.rules.lint.domain_import]"));
        let (stable, rerun) = sync_text(&linter.rules, &updated).unwrap();
        assert_eq!(stable, updated);
        assert_eq!(rerun.missing, Vec::<String>::new());
        assert_eq!(rerun.refreshed, Vec::<String>::new());
        assert!(!rerun.is_header_added);
    }

    #[test]
    fn existing_level_reason_and_exempt_are_untouched() {
        let linter = Linter::default();
        let original = format!("{HEADER}[lele.rules.lint.domain_import]\nlevel = \"off\"\nreason = \"legacy\"\n\n[[lele.rules.lint.domain_import.exempt]]\npath = \"src/legacy\"\nreason = \"mirror\"\n");
        let (updated, outcome) = sync_text(&linter.rules, &original).unwrap();
        assert!(!outcome.is_header_added);
        assert!(updated.contains("level = \"off\""));
        assert!(updated.contains("reason = \"legacy\""));
        assert!(updated.contains("path = \"src/legacy\""));
        assert!(outcome
            .missing
            .iter()
            .all(|name| name != "lele.rules.lint.domain_import"));
    }

    #[test]
    fn user_comment_above_project_survives() {
        let linter = Linter::default();
        let original = "# our team owns this file\n[lele.project]\nexclude = [\"target\"]\n";
        let (updated, _) = sync_text(&linter.rules, original).unwrap();
        assert!(updated.contains("# our team owns this file"));
        assert!(updated.contains("[lele.project]"));
    }

    #[test]
    fn stale_comment_line_is_refreshed() {
        let linter = Linter::default();
        let original = format!(
            "{HEADER}# LELE016 old words\n[lele.rules.lint.domain_import]\nlevel = \"deny\"\n"
        );
        let (updated, outcome) = sync_text(&linter.rules, &original).unwrap();
        assert_eq!(outcome.refreshed, vec!["lele.rules.lint.domain_import"]);
        assert!(!updated.contains("# LELE016 old words"));
        assert!(updated.contains("# LELE016 ·"));
    }

    #[test]
    fn every_comment_line_fits_in_120_chars() {
        let linter = Linter::default();
        let (updated, _) = sync_text(&linter.rules, "").unwrap();
        for line in updated.lines() {
            if line.starts_with("# LELE") {
                assert!(line.chars().count() <= 120, "{line}");
            }
        }
    }
}
