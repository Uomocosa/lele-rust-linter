use lele_lint::Project;
use lele_lint::traits::Rule;

pub(crate) fn rule_messages(rule: &dyn Rule, project: &Project) -> Vec<String> {
    rule.check(project)
        .into_iter()
        .map(|violation| violation.message)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::rule_messages;
    use crate::rules;
    use crate::testing;
    use lele_lint::Project;

    #[test]
    fn test_usage() {
        let project = testing::project_with(
            Project::default(),
            &[(
                "ui/root.rs",
                "pub fn setup(s: &mut S) { s.spawn((Node,)); }",
            )],
        );
        let messages = rule_messages(&rules::bevy_ui::BevyUi, &project);
        assert_eq!(messages.len(), 1);
    }
}
