use crate::rules;
use crate::BoundaryEntry;
use crate::RuleViolation;
use std::path::Path;

pub fn use_violation(
    file: &Path,
    line: usize,
    reported: &str,
    boundary: &BoundaryEntry,
) -> RuleViolation {
    let name = &boundary.name;
    let folders = boundary
        .folders
        .iter()
        .map(|folder| folder.display().to_string())
        .collect::<Vec<_>>()
        .join(", ");
    let why = &boundary.why;
    RuleViolation::from_site(
        file.to_path_buf(),
        line,
        0,
        rules::boundary_imports::BoundaryImports::CODE,
        format!("`{reported}` is not allowed in boundary \"{name}\" ({folders}) — {why}"),
    )
}

// no test_usage necessary
