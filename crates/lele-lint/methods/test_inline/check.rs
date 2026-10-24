use crate::rules;
use crate::EntryKind;
use crate::Project;
use crate::RuleViolation;

pub fn check(_self: &rules::test_inline::TestInline, project: &Project) -> Vec<RuleViolation> {
    let mut violations = Vec::new();

    for entry in &project.entries {
        if entry.kind != EntryKind::Directory {
            continue;
        }
        if rules::test_inline::TestInline::has_tests_component(&entry.relative_path) {
            violations.push(RuleViolation::from_file(entry.absolute_path.clone(), rules::test_inline::TestInline::CODE, format!(
                    "unit tests must be in the same file as the primary item — delete `{}` and move the tests inline",
                    entry.relative_path.display()
                )));
        }
    }

    violations
}

#[cfg(test)]
mod tests {
    use crate::rules;
    use std::path::Path;

    #[test]
    fn test_usage() {
        assert!(rules::test_inline::TestInline::has_tests_component(
            Path::new("player/tests")
        ));
        assert!(rules::test_inline::TestInline::has_tests_component(
            Path::new("player/tests/helpers.rs")
        ));
        assert!(!rules::test_inline::TestInline::has_tests_component(
            Path::new("player/mod.rs")
        ));
        assert!(!rules::test_inline::TestInline::has_tests_component(
            Path::new("bevy_systems")
        ));
    }
}
