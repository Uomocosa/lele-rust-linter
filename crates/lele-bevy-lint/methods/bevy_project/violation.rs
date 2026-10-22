use std::path::Path;

use lele_lint::Origin;
use lele_lint::RuleViolation;

use crate::scan;

pub fn violation(
    project: &scan::BevyProject,
    origin: Origin,
    rel_path: &Path,
    line: usize,
    code: &'static str,
    message: String,
) -> RuleViolation {
    let absolute = project.absolute_path(origin, rel_path);
    RuleViolation::site(absolute, line, 0, code, message)
}

#[cfg(test)]
mod tests {
    use super::violation;
    use crate::scan;
    use lele_lint::Location;
    use lele_lint::Origin;
    use lele_lint::Project;
    use std::path::Path;

    #[test]
    fn test_usage() {
        let project = Project::default();
        let found = violation(
            &scan::BevyProject(&project),
            Origin::Src,
            Path::new("a.rs"),
            7,
            "X",
            String::from("m"),
        );
        assert_eq!(found.code, "X");
        assert_eq!(found.location, Location::Site { line: 7, col: 0 });
    }
}
