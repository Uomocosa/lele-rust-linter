use crate::rules;
use crate::BoundaryEntry;
use crate::RuleViolation;

pub fn report_uses(
    file: &rules::boundary_imports::FileUse,
    boundary: &BoundaryEntry,
) -> Vec<RuleViolation> {
    let mut violations = Vec::new();
    for hit in &file.hits {
        if rules::boundary_imports::BoundaryImports::is_local(&hit.path) {
            continue;
        }
        let Some(reported) =
            rules::boundary_imports::BoundaryImports::forbidden_hit(hit, &boundary.cannot_use)
        else {
            continue;
        };
        violations.push(rules::boundary_imports::BoundaryImports::use_violation(
            &file.file, hit.line, &reported, boundary,
        ));
    }
    violations
}

// no test_usage necessary
