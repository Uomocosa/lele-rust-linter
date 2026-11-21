use std::path::PathBuf;

use crate::CommonArgs;
use crate::ErrorFormat;
use crate::SyncArgs;

pub(crate) fn lint_session_args(
    path: Option<PathBuf>,
    config: Option<PathBuf>,
    only: Option<Vec<String>>,
) -> CommonArgs {
    CommonArgs {
        error_format: ErrorFormat::Clippy,
        rule_list: false,
        explain: None,
        only,
        config,
        scan_folder: None,
        sync: SyncArgs {
            sync_config: false,
            check: false,
        },
        config_report: false,
        path,
    }
}

#[cfg(test)]
mod tests {
    use super::lint_session_args;

    #[test]
    fn test_usage() {
        let args = lint_session_args(None, None, None);
        assert!(!args.rule_list);
        assert!(!args.sync.sync_config);
    }
}
