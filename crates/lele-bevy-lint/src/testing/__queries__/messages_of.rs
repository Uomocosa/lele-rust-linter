use lele_lint::traits::Rule;

use crate::testing;

pub(crate) fn messages_of(rule: &dyn Rule, path: &str, source: &str) -> Vec<String> {
    testing::violations_of(rule, path, source)
        .into_iter()
        .map(|violation| violation.message)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::messages_of;
    use crate::rules;

    #[test]
    fn test_usage() {
        let found = messages_of(
            &rules::bevy_ui::BevyUi,
            "ui/root.rs",
            "pub fn setup(s: &mut S) { s.spawn((Node,)); }",
        );
        assert_eq!(found.len(), 1);
    }
}
