use crate::rules;
use crate::Project;
use crate::RuleViolation;

pub fn rule_check(
    _self: &rules::testing_layout::TestingLayout,
    project: &Project,
) -> Vec<RuleViolation> {
    let mut violations = Vec::new();

    for source in project.content_sources() {
        if !rules::testing_layout::TestingLayout::is_testing_path(source.relative_path) {
            continue;
        }
        let Some(component) =
            rules::testing_layout::TestingLayout::offending_component(source.relative_path)
        else {
            continue;
        };
        violations.push(RuleViolation::from_file(
            project.absolute_path(source.origin, source.relative_path),
            rules::testing_layout::TestingLayout::CODE,
            format!(
                "`src/testing/{component}` is outside the testing layout — `src/testing/` holds only `mod.rs`, `__basic__/` and the four roles `__app__/`, `__queries__/`, `__fixtures__/`, `__fakes__/`; move it into `__basic__/` or one of the four"
            ),
        ));
    }

    violations
}

#[cfg(test)]
mod tests {
    use super::rule_check;
    use crate::rules::testing_layout::TestingLayout;
    use crate::testing;
    use crate::Project;

    #[test]
    fn test_usage() {
        let project = testing::parsed_project(
            Project::default(),
            &[
                ("testing/mod.rs", "pub mod fixtures;"),
                ("testing/__fixtures__/row.rs", "pub struct Row;"),
                ("testing/__basic__/enums.rs", "pub enum Kind { A }"),
                ("testing/harness/lobby.rs", "pub struct Lobby;"),
            ],
        );
        let violations = rule_check(&TestingLayout, &project);
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message.contains("src/testing/harness"));
    }

    #[test]
    fn test_usage_reports_each_offending_file_once() {
        let project = testing::parsed_project(
            Project::default(),
            &[
                ("testing/harness/lobby.rs", ""),
                ("testing/harness/room.rs", ""),
                ("testing/keys.rs", ""),
            ],
        );
        let violations = rule_check(&TestingLayout, &project);
        assert_eq!(violations.len(), 3);
    }

    #[test]
    fn test_usage_allows_mod_and_four_roles_at_any_depth() {
        let project = testing::parsed_project(
            Project::default(),
            &[
                ("testing/mod.rs", "pub mod app;"),
                ("testing/__basic__/enums.rs", ""),
                ("testing/__app__/harness/checkpoint_board.rs", ""),
                ("testing/__queries__/find_one.rs", ""),
                ("testing/__fixtures__/__basic__/structs.rs", ""),
                ("testing/__fakes__/__traits__/kit.rs", ""),
            ],
        );
        assert!(rule_check(&TestingLayout, &project).is_empty());
    }
}
