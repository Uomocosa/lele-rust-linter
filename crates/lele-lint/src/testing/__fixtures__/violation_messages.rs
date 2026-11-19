use crate::testing;
use crate::traits::Rule;
use crate::Project;

pub(crate) fn violation_messages(rule: &dyn Rule, files: &[(&str, &str)]) -> Vec<String> {
    let project: Project = testing::parsed_project(testing::dunder_project(), files);
    let mut messages: Vec<String> = rule
        .check(&project)
        .into_iter()
        .map(|violation| violation.message)
        .collect();
    messages.sort();
    messages
}

#[cfg(test)]
mod tests {
    use super::violation_messages;
    use crate::rules;

    #[test]
    fn test_usage() {
        let messages = violation_messages(
            &rules::no_tests_in_plain_data::NoTestsInPlainData,
            &[(
                "config/settings.rs",
                "pub struct Settings { pub port: u16 }\n#[cfg(test)] mod tests { #[test] fn test_usage() {} }",
            )],
        );
        assert_eq!(messages.len(), 1);
    }
}
