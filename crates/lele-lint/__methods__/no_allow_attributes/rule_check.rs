use syn::visit::Visit;

use crate::rules;
use crate::Project;
use crate::RuleViolation;

pub fn rule_check(
    _self: &rules::no_allow_attributes::NoAllowAttributes,
    project: &Project,
) -> Vec<RuleViolation> {
    let mut violations = Vec::new();
    for source in project.content_sources() {
        let rel_path = source.relative_path;
        let file = source.file;
        let mut finder = rules::no_allow_attributes::AllowFinder {
            file: project.absolute_path(source.origin, rel_path),
            config_base: project.config_base(),
            clippy_allow_whitelist: &project.clippy_allow_whitelist,
            violations: Vec::new(),
        };
        finder.visit_file(file);
        violations.extend(finder.violations);
    }
    violations
}

#[cfg(test)]
mod tests {
    use super::rule_check;
    use crate::rules;
    use crate::testing;
    use crate::Project;

    #[test]
    fn test_usage() {
        let rule = rules::no_allow_attributes::NoAllowAttributes;
        let project = Project::default();
        assert!(rule_check(&rule, &project).is_empty());
    }

    #[test]
    fn test_usage_flags_allow() {
        assert_eq!(
            testing::no_allow_violation_count_without_whitelist("#[allow(dead_code)] struct Foo;"),
            1
        );
        assert_eq!(
            testing::no_allow_violation_count_without_whitelist(
                "struct Foo { #[allow(dead_code)] bar: u32 }"
            ),
            1
        );
        assert_eq!(
            testing::no_allow_violation_count_without_whitelist("#![allow(dead_code)]"),
            1
        );
        assert_eq!(
            testing::no_allow_violation_count_without_whitelist(
                "#[allow(clippy::missing_const_for_fn)] fn f() {}"
            ),
            1
        );
    }

    #[test]
    fn test_usage_flags_expect() {
        assert_eq!(
            testing::no_allow_violation_count_without_whitelist("#[expect(dead_code)] struct Foo;"),
            1
        );
    }

    #[test]
    fn test_usage_clean_passes() {
        assert_eq!(
            testing::no_allow_violation_count_without_whitelist("struct Foo { bar: u32 }"),
            0
        );
        assert_eq!(
            testing::no_allow_violation_count_without_whitelist(
                "#[derive(Clone)] struct Foo { bar: u32 }"
            ),
            0
        );
        assert_eq!(
            testing::no_allow_violation_count_without_whitelist("#[cfg(test)] mod tests {}"),
            0
        );
    }

    #[test]
    fn test_usage_whitelisted_passes() {
        let whitelist = vec![testing::allow_whitelist_entry(
            "unreachable_code",
            "src/x.rs",
            "derive macro",
        )];
        assert_eq!(
            testing::no_allow_violation_count("#![allow(unreachable_code)]", whitelist),
            0
        );
    }

    #[test]
    fn test_usage_whitelist_needs_matching_file() {
        let whitelist = vec![testing::allow_whitelist_entry(
            "unreachable_code",
            "src/other.rs",
            "derive macro",
        )];
        assert_eq!(
            testing::no_allow_violation_count("#![allow(unreachable_code)]", whitelist),
            1
        );
    }

    #[test]
    fn test_usage_whitelist_needs_reason() {
        let whitelist = vec![testing::allow_whitelist_entry(
            "unreachable_code",
            "src/x.rs",
            "   ",
        )];
        assert_eq!(
            testing::no_allow_violation_count("#![allow(unreachable_code)]", whitelist),
            1
        );
    }

    #[test]
    fn test_usage_whitelist_lists_all_lints() {
        let whitelist = vec![testing::allow_whitelist_entry(
            "unreachable_code",
            "src/x.rs",
            "derive macro",
        )];
        assert_eq!(
            testing::no_allow_violation_count(
                "#[allow(unreachable_code, dead_code)] struct Foo;",
                whitelist
            ),
            1
        );
    }
}
