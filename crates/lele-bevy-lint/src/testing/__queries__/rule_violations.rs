use lele_lint::Project;
use lele_lint::RuleViolation;
use lele_lint::traits::Rule;

pub(crate) fn rule_violations(rule: &dyn Rule, project: &Project) -> Vec<RuleViolation> {
    rule.check(project)
}

#[cfg(test)]
mod tests {
    use super::rule_violations;
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
        let violations = rule_violations(&rules::bevy_ui::BevyUi, &project);
        assert_eq!(violations.len(), 1);
        assert_eq!(violations[0].code, rules::bevy_ui::BevyUi::CODE);
    }
}
