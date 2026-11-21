use std::path::Path;

use crate::traits::Rule;
use crate::ConfigSync;
use crate::Error;
use crate::SyncOutcome;

pub fn synchronize(
    rules: &[Box<dyn Rule>],
    path: &Path,
    check: bool,
) -> Result<SyncOutcome, Error> {
    let original = match std::fs::read_to_string(path) {
        Ok(text) => text,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => String::new(),
        Err(error) => return Err(Error::Io(error)),
    };
    let (updated, outcome) = ConfigSync::sync_text(rules, &original)?;
    if check || updated == original {
        return Ok(outcome);
    }
    std::fs::write(path, updated).map_err(Error::Io)?;
    Ok(outcome)
}

#[cfg(test)]
mod tests {
    use super::synchronize;
    use crate::Linter;

    #[test]
    fn test_usage() {
        let linter = Linter::default();
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("lele.toml");
        let outcome = synchronize(&linter.rules, &path, false).unwrap();
        assert!(outcome.is_header_added);
        assert_ne!(outcome.missing, Vec::<String>::new());
        let reread = synchronize(&linter.rules, &path, true).unwrap();
        assert_eq!(reread.missing, Vec::<String>::new());
        assert_eq!(reread.refreshed, Vec::<String>::new());
        assert!(!reread.is_header_added);
    }

    #[test]
    fn check_on_stale_file_reports_without_writing() {
        let linter = Linter::default();
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("lele.toml");
        std::fs::write(&path, "[lele.project]\n").unwrap();
        let outcome = synchronize(&linter.rules, &path, true).unwrap();
        assert_ne!(outcome.missing, Vec::<String>::new());
        assert!(outcome.is_header_added);
        assert_eq!(std::fs::read_to_string(&path).unwrap(), "[lele.project]\n");
    }
}
