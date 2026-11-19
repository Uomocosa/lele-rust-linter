use lele_lint::RuleViolation;
use lele_lint::traits::Rule;

use crate::testing;

pub(crate) fn violations_of(rule: &dyn Rule, path: &str, source: &str) -> Vec<RuleViolation> {
    let project = testing::project_with(lele_lint::Project::default(), &[(path, source)]);
    testing::rule_violations(rule, &project)
}

#[cfg(test)]
mod tests {
    use super::violations_of;
    use crate::rules;

    #[test]
    fn test_usage() {
        let found = violations_of(
            &rules::bevy_ui::BevyUi,
            "ui/root.rs",
            "pub fn setup(s: &mut S) { s.spawn((Node,)); }",
        );
        assert_eq!(found.len(), 1);
    }
}
