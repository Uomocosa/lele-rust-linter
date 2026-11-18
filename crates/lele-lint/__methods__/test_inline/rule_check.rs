use crate::rules;
use crate::EntryKind;
use crate::Project;
use crate::RuleViolation;

pub fn rule_check(_self: &rules::test_inline::TestInline, project: &Project) -> Vec<RuleViolation> {
    let mut violations = Vec::new();

    for entry in &project.entries {
        if entry.kind != EntryKind::Directory {
            continue;
        }
        if rules::test_inline::TestInline::has_tests_component(&entry.relative_path) {
            violations.push(RuleViolation::from_file(entry.absolute_path.clone(), rules::test_inline::TestInline::CODE, format!(
                    "unit tests are in a separate file `{}` — move them into `#[cfg(test)] mod tests` of the file they test, then delete this file",
                    entry.relative_path.display()
                )));
        }
    }

    for entry in &project.entries {
        if entry.kind != EntryKind::File {
            continue;
        }
        if !rules::test_inline::TestInline::has_underscore_tests_stem(&entry.relative_path) {
            continue;
        }
        let has_tests = project
            .get_parsed(&entry.relative_path)
            .is_some_and(rules::test_inline::TestInline::file_has_test_items);
        if has_tests {
            violations.push(RuleViolation::from_file(entry.absolute_path.clone(), rules::test_inline::TestInline::CODE, format!(
                    "unit tests are in a separate file `{}` — colocate: move the tests into `#[cfg(test)] mod tests` of the file they test, then delete this file",
                    entry.relative_path.display()
                )));
        }
    }

    let tests_dir = project.root.join("tests");
    if tests_dir.is_dir() && !rules::test_inline::TestInline::is_listed_e2e_dir(project, &tests_dir)
    {
        violations.push(RuleViolation::from_file(tests_dir, rules::test_inline::TestInline::CODE, String::from(
                "crate-level `tests/` directory holds integration/e2e tests outside the crate — it must be listed in `[lele.config] e2e_test_dirs` (paths relative to the config file); unit tests go colocated",
            )));
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
            Path::new("__bevy_systems__")
        ));
    }
}
