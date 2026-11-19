use std::path::PathBuf;

use crate::AllowWhitelistEntry;

pub(crate) fn allow_whitelist_entry(allow: &str, file: &str, reason: &str) -> AllowWhitelistEntry {
    AllowWhitelistEntry {
        allow: allow.to_string(),
        file: PathBuf::from(file),
        reason: reason.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::allow_whitelist_entry;

    #[test]
    fn test_usage() {
        assert_eq!(
            allow_whitelist_entry("dead_code", "src/x.rs", "why").allow,
            "dead_code"
        );
    }
}
