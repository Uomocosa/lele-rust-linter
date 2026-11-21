use std::path::Path;

use atomic_delegate_macros::atomic_delegates;
use derive_more::Deref;

use crate::CommonArgs;
use crate::Config;
use crate::Error;
use crate::Linter;
use crate::Project;
use crate::RuleSnapshot;

#[derive(Deref)]
pub struct LintSession {
    pub binary: &'static str,
}

#[atomic_delegates]
impl LintSession {
    pub fn run(
        &self,
        linter: &mut Linter,
        args: &CommonArgs,
        sync_methods: bool,
        prepare_project: impl FnOnce(&mut Project, &Path) -> Result<(), Error>,
        report_project: impl FnOnce(&Project) -> Option<i32>,
    ) -> i32 {
    }
}

#[atomic_delegates]
impl LintSession {
    pub(crate) fn load_project(&self, args: &CommonArgs) -> Result<Project, i32> {}
    pub(crate) fn run_sync_config(
        &self,
        linter: &Linter,
        args: &CommonArgs,
        project: &Project,
    ) -> Option<i32> {
    }
    pub(crate) fn load_config(
        &self,
        args: &CommonArgs,
        project: &mut Project,
        prepare_project: impl FnOnce(&mut Project, &Path) -> Result<(), Error>,
    ) -> Result<Config, i32> {
    }
    pub(crate) fn collect_snapshot(linter: &Linter) -> Vec<RuleSnapshot> {}
    pub(crate) fn filter_report_snapshot(
        snapshot: &[RuleSnapshot],
        codes: Option<&[String]>,
    ) -> Result<Vec<RuleSnapshot>, Vec<String>> {
    }
    pub(crate) fn parse_failure_messages(&self, project: &Project) -> Vec<String> {}
}

#[cfg(test)]
mod tests {
    use super::LintSession;

    #[test]
    fn test_usage() {
        let session = LintSession {
            binary: "lele_lint",
        };
        assert_eq!(session.binary, "lele_lint");
    }
}
