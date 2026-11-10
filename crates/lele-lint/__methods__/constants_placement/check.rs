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
        let project = Project::default();
        let users = set(&["rules/a.rs", "rules/b.rs"]);
        assert_eq!(
            rules::constants_placement::ConstantsPlacement::common_ancestor_dir(&users, &project),
            PathBuf::from("rules")
        );
    }

    #[test]
    fn test_usage_common_ancestor_spans_top() {
        let project = Project::default();
        let users = set(&["rules/a.rs", "project/b.rs"]);
        assert_eq!(
            rules::constants_placement::ConstantsPlacement::common_ancestor_dir(&users, &project),
            PathBuf::new()
        );
    }

    #[test]
    fn test_usage_common_ancestor_lifts_nested() {
        let project = Project::default();
        let users = set(&["rules/sub/a.rs", "rules/other/b.rs"]);
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
        let nested = set(&["foo/__traits__/user.rs"]);
        assert_eq!(
            rules::constants_placement::ConstantsPlacement::common_ancestor_dir(&nested, &project),
            PathBuf::from("foo")
        );
        let root = set(&["__traits__/user.rs"]);
        assert_eq!(
            rules::constants_placement::ConstantsPlacement::common_ancestor_dir(&root, &project),
            PathBuf::new()
        );
        let systems = set(&["foo/__bevy_systems__/poll.rs"]);
        assert_eq!(
            rules::constants_placement::ConstantsPlacement::common_ancestor_dir(&systems, &project),
            PathBuf::from("foo")
        );
    }

    fn const_messages(files: &[(&str, &str)]) -> Vec<String> {
        let mut project = Project {
            dunder_paths: DunderPath::defaults(),
            ..Project::default()
        };
        let mut violations = Vec::new();
        for (path, source) in files {
            let file: syn::File = syn::parse_str(source).unwrap();
            project.parsed_files.insert(PathBuf::from(path), file);
        }
        for violation in check(&rules::constants_placement::ConstantsPlacement, &project) {
            violations.push(violation.message);
        }
        violations.sort();
        violations
    }

    fn user_source(const_name: &str) -> String {
        format!(
            "use crate::basic::constants::{const_name};\n\npub trait User {{ fn call(&self) -> String {{ crate::basic::constants::{const_name}.len().to_string() }} }}\n"
        )
    }

    #[test]
    fn test_usage_root_const_used_only_by_code_dunder_fine_stays_clean() {
        assert_eq!(
            const_messages(&[
                (
                    "constants.rs",
                    "pub const ACRONYMS: &[&str] = &[\"ABC\"];\n"
                ),
                (
                    "__traits__/to_snake_case.rs",
                    user_source("ACRONYMS").as_str(),
                ),
            ]),
            Vec::<String>::new()
        );
    }

    #[test]
    fn test_usage_root_const_used_only_by_nested_code_dunder_lowers_to_its_domain() {
        let found = const_messages(&[
            (
                "constants.rs",
                "pub const ACRONYMS: &[&str] = &[\"ABC\"];\n",
            ),
            (
                "foo/__traits__/to_snake_case.rs",
                user_source("ACRONYMS").as_str(),
            ),
        ]);
        assert_eq!(found.len(), 1);
        assert_eq!(
            found[0],
            "const `ACRONYMS` lives at root but is only used under `foo` — lower it to `foo/__basic__/constants.rs`"
        );
    }

    #[test]
    fn test_usage_const_used_across_code_dunders_lifts_to_root() {
        let found = const_messages(&[
            (
                "foo/constants.rs",
                "pub const ACRONYMS: &[&str] = &[\"ABC\"];\n",
            ),
            (
                "__traits__/to_snake_case.rs",
                user_source("ACRONYMS").as_str(),
            ),
        ]);
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
