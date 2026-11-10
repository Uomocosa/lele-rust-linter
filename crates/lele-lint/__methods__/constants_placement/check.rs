use std::collections::HashSet;

use crate::rules;
use crate::Project;
use crate::RuleViolation;

pub fn check(
    _self: &rules::constants_placement::ConstantsPlacement,
    project: &Project,
) -> Vec<RuleViolation> {
    let mut violations = Vec::new();
    let defined = rules::constants_placement::ConstantsPlacement::collect_defined_consts(
        &project.parsed_files,
    );
    let defined_names: HashSet<String> = defined.keys().cloned().collect();
    let usages = rules::constants_placement::ConstantsPlacement::collect_const_usages(
        &project.parsed_files,
        &defined_names,
    );

    for (name, rel_path) in &defined {
        if let Some(users) = usages.get(name) {
            violations.extend(
                rules::constants_placement::ConstantsPlacement::check_fn_file_const(
                    name, rel_path, project,
                ),
            );
            violations.extend(
                rules::constants_placement::ConstantsPlacement::check_lift_to_ancestor(
                    name, rel_path, users, project,
                ),
            );
            violations.extend(
                rules::constants_placement::ConstantsPlacement::check_lower_to_subdir(
                    name, rel_path, users, project,
                ),
            );
        }
    }

    violations
}

#[cfg(test)]
mod tests {
    use super::check;
    use crate::rules;
    use crate::DunderPath;
    use crate::Project;
    use std::collections::HashSet;
    use std::path::PathBuf;

    fn set(paths: &[&str]) -> HashSet<PathBuf> {
        paths.iter().map(PathBuf::from).collect()
    }

    #[test]
    fn test_usage() {
        let rule = rules::constants_placement::ConstantsPlacement;
        let project = Project::default();
        assert!(check(&rule, &project).is_empty());
    }

    #[test]
    fn test_usage_sibling_constants() {
        let project = Project {
            dunder_paths: DunderPath::defaults(),
            ..Project::default()
        };
        assert_eq!(
            rules::constants_placement::ConstantsPlacement::sibling_constants(
                &PathBuf::from("discover.rs"),
                &project
            ),
            PathBuf::from("__basic__/constants.rs")
        );
        assert_eq!(
            rules::constants_placement::ConstantsPlacement::sibling_constants(
                &PathBuf::from("rules/foo.rs"),
                &project
            ),
            PathBuf::from("rules/__basic__/constants.rs")
        );
    }

    #[test]
    fn test_usage_common_prefix() {
        assert_eq!(
            rules::constants_placement::ConstantsPlacement::common_prefix(
                &PathBuf::from("rules/sub"),
                &PathBuf::from("rules/other")
            ),
            PathBuf::from("rules")
        );
        assert_eq!(
            rules::constants_placement::ConstantsPlacement::common_prefix(
                &PathBuf::from("rules"),
                &PathBuf::from("project")
            ),
            PathBuf::new()
        );
    }

    #[test]
    fn test_usage_common_ancestor_single_dir() {
        let users = set(&["rules/a.rs", "rules/b.rs"]);
        assert_eq!(
            rules::constants_placement::ConstantsPlacement::common_ancestor_dir(&users),
            PathBuf::from("rules")
        );
    }

    #[test]
    fn test_usage_common_ancestor_spans_top() {
        let users = set(&["rules/a.rs", "project/b.rs"]);
        assert_eq!(
            rules::constants_placement::ConstantsPlacement::common_ancestor_dir(&users),
            PathBuf::new()
        );
    }

    #[test]
    fn test_usage_common_ancestor_lifts_nested() {
        let users = set(&["rules/sub/a.rs", "rules/other/b.rs"]);
        assert_eq!(
            rules::constants_placement::ConstantsPlacement::common_ancestor_dir(&users),
            PathBuf::from("rules")
        );
    }

    #[test]
    fn test_usage_root_const_is_not_lifted_to_itself() {
        let project = Project {
            dunder_paths: DunderPath::defaults(),
            ..Project::default()
        };
        let users = set(&["toolchain.rs"]);
        assert!(
            rules::constants_placement::ConstantsPlacement::check_lift_to_ancestor(
                "CACHE_TARGET_DIR",
                &PathBuf::from("__basic__/constants.rs"),
                &users,
                &project,
            )
            .is_empty()
        );
    }

    #[test]
    fn test_usage_domain_const_is_lifted_to_root() {
        let project = Project {
            dunder_paths: DunderPath::defaults(),
            ..Project::default()
        };
        let users = set(&["toolchain.rs", "driver/mod.rs"]);
        assert_eq!(
            rules::constants_placement::ConstantsPlacement::check_lift_to_ancestor(
                "CACHE_TARGET_DIR",
                &PathBuf::from("toolchain/__basic__/constants.rs"),
                &users,
                &project,
            )
            .len(),
            1
        );
    }
}
