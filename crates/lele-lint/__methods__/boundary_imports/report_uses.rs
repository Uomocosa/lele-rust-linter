use crate::rules;
use crate::BoundaryEntry;
use crate::Project;
use crate::RuleViolation;

pub fn report_uses(
    project: &Project,
    file: &rules::boundary_imports::FileUse,
    boundary: &BoundaryEntry,
) -> Vec<RuleViolation> {
    let mut violations = Vec::new();
    for hit in &file.hits {
        if rules::boundary_imports::BoundaryImports::is_local(&hit.path) {
            let Some(absolute) = rules::boundary_imports::BoundaryImports::resolve_local_hit(
                project,
                &file.crate_rel,
                &hit.path,
            ) else {
                continue;
            };
            let resolved = rules::boundary_imports::PathHit {
                path: absolute,
                line: hit.line,
                glob: hit.glob,
            };
            let Some(reported) = rules::boundary_imports::BoundaryImports::forbidden_crate_hit(
                &resolved,
                &boundary.cannot_use,
            ) else {
                continue;
            };
            violations.push(rules::boundary_imports::BoundaryImports::use_violation(
                &file.file, hit.line, &reported, boundary,
            ));
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

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::report_uses;
    use crate::rules;
    use crate::BoundaryEntry;
    use crate::Project;

    #[test]
    fn test_usage() {
        let project = Project::default();
        let file = rules::boundary_imports::FileUse {
            file: PathBuf::from("src/session/session.rs"),
            crate_rel: PathBuf::from("src/session/session.rs"),
            hits: vec![rules::boundary_imports::PathHit {
                path: "crate::ui::Thing".to_string(),
                line: 1,
                glob: false,
            }],
        };
        let boundary = BoundaryEntry {
            name: "session is pure".to_string(),
            why: "no io".to_string(),
            folders: vec![PathBuf::from("src/session")],
            cannot_use: vec!["crate::ui".to_string()],
            require: None,
        };
        let violations = report_uses(&project, &file, &boundary);
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message.contains("crate::ui::Thing"));
        let external = BoundaryEntry {
            cannot_use: vec!["std::time".to_string()],
            ..boundary
        };
        assert!(report_uses(&project, &file, &external).is_empty());
    }
}
