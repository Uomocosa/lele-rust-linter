use crate::checkers;
use crate::EntryKind;
use crate::Project;
use crate::RuleViolation;

pub fn check(_self: &checkers::test_inline::TestInline, project: &Project) -> Vec<RuleViolation> {
    let mut violations = Vec::new();

    for entry in &project.entries {
        if entry.kind != EntryKind::Directory {
            continue;
        }
        if has_tests_component(&entry.relative_path) {
            violations.push(RuleViolation::from_file(entry.absolute_path.clone(), checkers::test_inline::TestInline::CODE, format!(
                    "unit tests must be in the same file as the primary item — delete `{}` and move the tests inline",
                    entry.relative_path.display()
                )));
        }
    }

    violations
}

fn has_tests_component(path: &std::path::Path) -> bool {
    path.components()
        .any(|c| c.as_os_str().to_str().is_some_and(|s| s == "tests"))
}

#[cfg(test)]
mod tests {
    use super::has_tests_component;
    use std::path::Path;

    #[test]
    fn test_usage() {
        assert!(has_tests_component(Path::new("player/tests")));
        assert!(has_tests_component(Path::new("player/tests/helpers.rs")));
        assert!(!has_tests_component(Path::new("player/mod.rs")));
        assert!(!has_tests_component(Path::new("bevy_systems")));
    }
}
