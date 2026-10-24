use crate::rules;
use crate::BoundaryEntry;
use crate::RuleViolation;
use std::path::Path;

pub fn validate_boundary(boundary: &BoundaryEntry, base: &Path) -> Vec<RuleViolation> {
    let mut violations = Vec::new();
    if boundary.folders.is_empty() {
        violations.push(RuleViolation::from_file(
            base.join("lele.toml"),
            rules::boundary_imports::BoundaryImports::CODE,
            format!("boundary `{}` has no folders", boundary.name),
        ));
    }
    for folder in &boundary.folders {
        if !base.join(folder).is_dir() {
            violations.push(RuleViolation::from_file(
                base.join("lele.toml"),
                rules::boundary_imports::BoundaryImports::CODE,
                format!(
                    "boundary `{}` names folder `{}` which does not exist",
                    boundary.name,
                    folder.display()
                ),
            ));
        }
    }
    if boundary.cannot_use.is_empty() && boundary.require.is_none() {
        violations.push(RuleViolation::from_file(
            base.join("lele.toml"),
            rules::boundary_imports::BoundaryImports::CODE,
            format!(
                "boundary `{}` needs a non-empty `cannot_use` or a `require`",
                boundary.name
            ),
        ));
    }
    violations
}

// no test_usage necessary
