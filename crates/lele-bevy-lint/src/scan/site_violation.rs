use std::path::Path;

use lele_lint::Origin;
use lele_lint::Project;
use lele_lint::RuleViolation;

#[must_use]
pub fn site_violation(
    project: &Project,
    origin: Origin,
    rel_path: &Path,
    line: usize,
    code: &'static str,
    message: String,
) -> RuleViolation {
    RuleViolation::site(
        project.absolute_path(origin, rel_path),
        line,
        0,
        code,
        message,
    )
}

#[cfg(test)]
mod tests {
    use super::site_violation;
    use lele_lint::Location;
    use lele_lint::Origin;
    use lele_lint::Project;
    use std::path::Path;

    #[test]
    fn test_usage() {
        let violation = site_violation(
            &Project::default(),
            Origin::Src,
            Path::new("a.rs"),
            7,
            "E029",
            String::from("m"),
        );
        assert_eq!(violation.code, "E029");
        assert_eq!(violation.location, Location::Site { line: 7, col: 0 });
    }
}
