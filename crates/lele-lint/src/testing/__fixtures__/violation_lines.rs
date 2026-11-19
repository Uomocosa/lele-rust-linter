use crate::testing;
use crate::traits::Rule;
use crate::Location;
use crate::Project;

pub(crate) fn violation_lines(rule: &dyn Rule, files: &[(&str, &str)]) -> Vec<usize> {
    let project: Project = testing::parsed_project(Project::default(), files);
    let mut lines: Vec<usize> = rule
        .check(&project)
        .into_iter()
        .map(|violation| match violation.location {
            Location::Site { line, .. } => line,
            Location::File => 0,
        })
        .collect();
    lines.sort_unstable();
    lines
}

#[cfg(test)]
mod tests {
    use super::violation_lines;
    use crate::rules;

    #[test]
    fn test_usage() {
        let lines = violation_lines(
            &rules::no_tests_in_plain_data::NoTestsInPlainData,
            &[(
                "config/settings.rs",
                "pub struct Settings { pub port: u16 }\n#[cfg(test)] mod tests { #[test] fn test_usage() {} }",
            )],
        );
        assert_eq!(lines.len(), 1);
    }
}
