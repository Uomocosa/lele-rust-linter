use std::collections::HashSet;

use crate::rules;
use crate::Project;
use crate::RuleViolation;

pub fn rule_check(
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
    use super::rule_check;
    use crate::rules;
    use crate::testing;
    use crate::DunderPath;
    use crate::Project;
    use std::path::PathBuf;

    #[test]
    fn test_usage() {
        let rule = rules::constants_placement::ConstantsPlacement;
        let project = Project::default();
        assert!(rule_check(&rule, &project).is_empty());
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
        let project = Project::default();
        let users = testing::path_set(&["rules/a.rs", "rules/b.rs"]);
        assert_eq!(
            rules::constants_placement::ConstantsPlacement::common_ancestor_dir(&users, &project),
            PathBuf::from("rules")
        );
    }

    #[test]
    fn test_usage_common_ancestor_spans_top() {
        let project = Project::default();
        let users = testing::path_set(&["rules/a.rs", "project/b.rs"]);
        assert_eq!(
            rules::constants_placement::ConstantsPlacement::common_ancestor_dir(&users, &project),
            PathBuf::new()
        );
    }

    #[test]
    fn test_usage_common_ancestor_lifts_nested() {
        let project = Project::default();
        let users = testing::path_set(&["rules/sub/a.rs", "rules/other/b.rs"]);
        assert_eq!(
            rules::constants_placement::ConstantsPlacement::common_ancestor_dir(&users, &project),
            PathBuf::from("rules")
        );
    }

    #[test]
    fn test_usage_common_ancestor_code_dunder_users_belong_to_their_parent_domain() {
        let project = Project {
            dunder_paths: DunderPath::defaults(),
            ..Project::default()
        };
        let nested = testing::path_set(&["foo/__traits__/user.rs"]);
        assert_eq!(
            rules::constants_placement::ConstantsPlacement::common_ancestor_dir(&nested, &project),
            PathBuf::from("foo")
        );
        let root = testing::path_set(&["__traits__/user.rs"]);
        assert_eq!(
            rules::constants_placement::ConstantsPlacement::common_ancestor_dir(&root, &project),
            PathBuf::new()
        );
        let systems = testing::path_set(&["foo/__bevy_systems__/poll.rs"]);
        assert_eq!(
            rules::constants_placement::ConstantsPlacement::common_ancestor_dir(&systems, &project),
            PathBuf::from("foo")
        );
    }

    #[test]
    fn test_usage_root_const_used_only_by_code_dunder_fine_stays_clean() {
        assert_eq!(
            testing::violation_messages(
                &rules::constants_placement::ConstantsPlacement,
                &[
                    (
                        "constants.rs",
                        "pub const ACRONYMS: &[&str] = &[\"ABC\"];\n"
                    ),
                    (
                        "__traits__/to_snake_case.rs",
                        testing::constant_user_source("ACRONYMS").as_str(),
                    ),
                ]
            ),
            Vec::<String>::new()
        );
    }

    #[test]
    fn test_usage_root_const_used_only_by_nested_code_dunder_lowers_to_its_domain() {
        let found = testing::violation_messages(
            &rules::constants_placement::ConstantsPlacement,
            &[
                (
                    "constants.rs",
                    "pub const ACRONYMS: &[&str] = &[\"ABC\"];\n",
                ),
                (
                    "foo/__traits__/to_snake_case.rs",
                    testing::constant_user_source("ACRONYMS").as_str(),
                ),
            ],
        );
        assert_eq!(found.len(), 1);
        assert_eq!(
            found[0],
            "const `ACRONYMS` lives at root but is only used under `foo` — lower it to `foo/__basic__/constants.rs`"
        );
    }

    #[test]
    fn test_usage_const_used_across_code_dunders_lifts_to_root() {
        let found = testing::violation_messages(
            &rules::constants_placement::ConstantsPlacement,
            &[
                (
                    "foo/constants.rs",
                    "pub const ACRONYMS: &[&str] = &[\"ABC\"];\n",
                ),
                (
                    "__traits__/to_snake_case.rs",
                    testing::constant_user_source("ACRONYMS").as_str(),
                ),
            ],
        );
        assert_eq!(found.len(), 1);
        assert_eq!(
            found[0],
            "const `ACRONYMS` is used across top-level dirs but lives in `foo/constants.rs` — lift it to `src/__basic__/constants.rs`"
        );
    }

    #[test]
    fn test_usage_root_const_is_not_lifted_to_itself() {
        let project = Project {
            dunder_paths: DunderPath::defaults(),
            ..Project::default()
        };
        let users = testing::path_set(&["toolchain.rs"]);
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
        let users = testing::path_set(&["toolchain.rs", "driver/mod.rs"]);
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
