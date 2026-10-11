use std::path::PathBuf;

use crate::Location;

#[derive(Debug)]
pub struct RuleViolation {
    pub file: PathBuf,
    pub location: Location,
    pub code: &'static str,
    pub message: String,
}

impl RuleViolation {
    #[must_use]
    pub fn file(file: PathBuf, code: &'static str, message: String) -> Self {
        Self {
            file,
            location: Location::File,
            code,
            message,
        }
    }

    #[must_use]
    pub fn site(
        file: PathBuf,
        line: usize,
        col: usize,
        code: &'static str,
        message: String,
    ) -> Self {
        Self {
            file,
            location: Location::Site { line, col },
            code,
            message,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::RuleViolation;
    use crate::Location;

    #[test]
    fn test_usage() {
        let file_level = RuleViolation::file(PathBuf::from("a.rs"), "E001", String::from("m"));
        assert_eq!(file_level.location, Location::File);
        let site_level =
            RuleViolation::site(PathBuf::from("a.rs"), 7, 3, "E002", String::from("m"));
        assert_eq!(site_level.location, Location::Site { line: 7, col: 3 });
    }
}
