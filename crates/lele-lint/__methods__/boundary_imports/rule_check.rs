use crate::rules;
use crate::Project;
use crate::RuleViolation;

pub fn rule_check(
    _self: &rules::boundary_imports::BoundaryImports,
    project: &Project,
) -> Vec<RuleViolation> {
    let mut violations = Vec::new();
    if project.boundaries.is_empty() {
        return violations;
    }
    let base = project.config_base();
    let prefix = rules::boundary_imports::BoundaryImports::base_prefix(project, &base);
    let stem_map = rules::boundary_imports::BoundaryImports::src_stem_map(project);
    let files = rules::boundary_imports::BoundaryImports::collect_file_uses(project);
    for boundary in &project.boundaries {
        violations.extend(rules::boundary_imports::BoundaryImports::validate_boundary(
            boundary, &base,
        ));
        if boundary.cannot_use.is_empty() {
            continue;
        }
        for file in &files {
            if !rules::boundary_imports::BoundaryImports::file_in_boundary(
                project,
                &file.crate_rel,
                &prefix,
                &boundary.folders,
                &stem_map,
            ) {
                continue;
            }
            violations.extend(rules::boundary_imports::BoundaryImports::report_uses(
                project, file, boundary,
            ));
        }
    }
    violations
}

#[cfg(test)]
mod tests {
    use crate::rules;
    use crate::testing;
    use crate::BoundaryEntry;
    use crate::Project;
    use std::collections::HashMap;
    use std::path::PathBuf;

    use super::rule_check;

    #[test]
    fn test_usage() {
        let project = Project {
            boundaries: vec![BoundaryEntry {
                name: "core".to_string(),
                why: "pure".to_string(),
                folders: vec![PathBuf::from("src")],
                cannot_use: vec!["std::time".to_string()],
                require: None,
            }],
            ..testing::parsed_project(
                testing::relative_project(),
                &[("core/clock.rs", "use std::time::Instant;\n")],
            )
        };
        let violations = rule_check(&rules::boundary_imports::BoundaryImports, &project);
        assert!(violations
            .iter()
            .any(|violation| violation.message.contains("std::time")));
    }

    #[test]
    fn local_paths_and_hit_matching() {
        assert!(rules::boundary_imports::BoundaryImports::is_local(
            "crate::foo"
        ));
        assert!(rules::boundary_imports::BoundaryImports::is_local(
            "self::foo"
        ));
        assert!(rules::boundary_imports::BoundaryImports::is_local(
            "super::foo"
        ));
        assert!(!rules::boundary_imports::BoundaryImports::is_local(
            "std::time"
        ));

        let direct = rules::boundary_imports::PathHit {
            path: "std::time::Instant".to_string(),
            line: 1,
            glob: false,
        };
        assert!(rules::boundary_imports::BoundaryImports::matches_hit(
            "std::time",
            &direct
        ));
        assert!(!rules::boundary_imports::BoundaryImports::matches_hit(
            "std::timex",
            &direct
        ));

        let glob = rules::boundary_imports::PathHit {
            path: "std::time".to_string(),
            line: 1,
            glob: true,
        };
        assert!(rules::boundary_imports::BoundaryImports::matches_hit(
            "std::time::Instant",
            &glob
        ));

        let mut aliases = HashMap::new();
        aliases.insert("time".to_string(), "std::time".to_string());
        assert_eq!(
            rules::boundary_imports::BoundaryImports::expand("time::Instant::now", &aliases),
            "std::time::Instant::now"
        );
        assert_eq!(
            rules::boundary_imports::BoundaryImports::expand("time", &aliases),
            "std::time"
        );
        assert_eq!(
            rules::boundary_imports::BoundaryImports::expand("std::fs::read", &aliases),
            "std::fs::read"
        );
    }
}
