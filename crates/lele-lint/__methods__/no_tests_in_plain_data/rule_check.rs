use crate::rules;
use crate::Origin;
use crate::Project;
use crate::RuleViolation;

pub fn rule_check(
    _self: &rules::no_tests_in_plain_data::NoTestsInPlainData,
    project: &Project,
) -> Vec<RuleViolation> {
    let mut violations = Vec::new();

    for source in project.content_sources() {
        if matches!(source.origin, Origin::Examples) {
            continue;
        }
        let rel_path = source.relative_path;
        if !rules::test_usage::TestUsage::is_forbidden_file(rel_path, source.file) {
            continue;
        }
        for item in &source.file.items {
            let Some(line) = rules::no_dunder_tests::NoDunderTests::cfg_test_line(item) else {
                continue;
            };
            violations.push(RuleViolation::from_site(project.absolute_path(source.origin, rel_path), line, 0, rules::no_tests_in_plain_data::NoTestsInPlainData::CODE, format!(
                    "`#[cfg(test)]` is not allowed in plain-data file `{}` — delete the test block and test the method or function that uses this data instead",
                    rel_path.display()
                )));
        }
    }

    violations
}

#[cfg(test)]
mod tests {
    use crate::DunderPath;
    use std::path::PathBuf;

    use super::rule_check;
    use crate::rules::no_tests_in_plain_data::NoTestsInPlainData;
    use crate::Project;

    fn project(files: &[(&str, &str)]) -> Project {
        let mut project = Project {
            dunder_paths: DunderPath::defaults(),
            ..Project::default()
        };
        for (path, source) in files {
            let file: syn::File = syn::parse_str(source).unwrap();
            project.parsed_files.insert(PathBuf::from(path), file);
        }
        project
    }

    #[test]
    fn test_usage() {
        let project = project(&[(
            "config/settings.rs",
            "pub struct Settings { pub port: u16 }\n#[cfg(test)] mod tests { #[test] fn test_usage() {} }",
        )]);
        let violations = rule_check(&NoTestsInPlainData, &project);
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message.contains("plain-data"));
    }

    #[test]
    fn test_usage_plugin_impl_is_allowed() {
        let project = project(&[(
            "config/registry.rs",
            "pub struct RegistryPlugin;\nimpl Plugin for RegistryPlugin { fn build(&self, app: &mut App) {} }\n#[cfg(test)] mod tests { #[test] fn test_usage() {} }",
        )]);
        assert!(rule_check(&NoTestsInPlainData, &project).is_empty());
    }

    #[test]
    fn test_usage_systems_file_with_test_is_allowed() {
        let project = project(&[(
            "enemy/__bevy_systems__/tick.rs",
            "pub fn tick() {}\n\n#[cfg(test)] mod tests { #[test] fn test_usage() {} }",
        )]);
        assert!(rule_check(&NoTestsInPlainData, &project).is_empty());
    }

    #[test]
    fn test_usage_behaviour_file_is_allowed() {
        let project = project(&[(
            "config/guest.rs",
            "pub struct Guest(pub String);\nimpl Guest { pub fn greet(&self) -> String { format!(\"hi\") } }\n#[cfg(test)] mod tests { #[test] fn test_usage() {} }",
        )]);
        assert!(rule_check(&NoTestsInPlainData, &project).is_empty());
    }
}
