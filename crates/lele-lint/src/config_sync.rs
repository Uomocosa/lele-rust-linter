use std::path::Path;
use std::path::PathBuf;

use atomic_delegate_macros::atomic_delegates;

use crate::traits::Rule;
use crate::ConfigReportRow;
use crate::Error;
use crate::Project;
use crate::RelaxedRule;
use crate::RuleSnapshot;
use crate::SyncOutcome;

pub struct ConfigSync;

#[atomic_delegates]
impl ConfigSync {
    pub fn config_path(explicit: Option<&Path>, project: &Project) -> PathBuf {}
    pub fn synchronize(
        rules: &[Box<dyn Rule>],
        path: &Path,
        check: bool,
    ) -> Result<SyncOutcome, Error> {
    }
    pub fn render_sync_check(outcome: &SyncOutcome, binary: &str) -> String {}
    pub fn render_config_report(snapshot: &[RuleSnapshot], relaxed: &[RelaxedRule]) -> String {}
}

#[atomic_delegates]
impl ConfigSync {
    pub(crate) fn sync_text(
        rules: &[Box<dyn Rule>],
        original: &str,
    ) -> Result<(String, SyncOutcome), Error> {
    }
    pub(crate) fn ensure_rule_table(
        doc: &mut toml_edit::DocumentMut,
        rule: &dyn Rule,
        outcome: &mut SyncOutcome,
    ) {
    }
    pub(crate) fn child_table<'table>(
        table: &'table mut toml_edit::Table,
        key: &str,
    ) -> Option<&'table mut toml_edit::Table> {
    }
    pub(crate) fn first_sentence(summary: &str, max_chars: usize) -> String {}
    pub(crate) fn report_row(entry: &RuleSnapshot, relaxed: &[RelaxedRule]) -> ConfigReportRow {}
}

#[cfg(test)]
mod tests {
    use super::ConfigSync;
    use crate::SyncOutcome;

    #[test]
    fn test_usage() {
        let report = ConfigSync::render_config_report(&[], &[]);
        assert!(report.contains("CODE"));
        assert_eq!(
            ConfigSync::render_sync_check(&SyncOutcome::default(), "lele-lint"),
            ""
        );
    }
}
