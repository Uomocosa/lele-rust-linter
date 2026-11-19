use std::path::Path;

use lele_lint::Origin;
use lele_lint::Project;
use lele_lint::RuleViolation;

use crate::scan;

pub(crate) fn preview_violation() -> RuleViolation {
    let project = Project::default();
    scan::BevyProject(&project).violation(
        Origin::Src,
        Path::new("a.rs"),
        1,
        "X",
        String::from("no preview"),
    )
}

#[cfg(test)]
mod tests {
    use super::preview_violation;

    #[test]
    fn test_usage() {
        assert_eq!(preview_violation().code, "X");
    }
}
