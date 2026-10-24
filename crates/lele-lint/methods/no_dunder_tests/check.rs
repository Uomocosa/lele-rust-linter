use crate::rules;
use crate::Project;
use crate::RuleViolation;

pub fn check(
    _self: &rules::no_dunder_tests::NoDunderTests,
    project: &Project,
) -> Vec<RuleViolation> {
    let mut violations = Vec::new();

    for source in project.content_sources() {
        let rel_path = source.relative_path;
        let file = source.file;
        if !project.is_in_dunder_dir(rel_path) {
            continue;
        }
        for item in &file.items {
            let Some(line) = rules::no_dunder_tests::NoDunderTests::cfg_test_line(item) else {
                continue;
            };
            violations.push(RuleViolation::from_site(project.absolute_path(source.origin, rel_path), line, 0, rules::no_dunder_tests::NoDunderTests::CODE, "`#[cfg(test)]` is not allowed in a `__basic__` container — cover the function or method that uses these types instead"
                    .to_string()));
        }
    }

    violations
}

#[cfg(test)]
mod tests {
    use crate::DunderPath;
    use std::path::PathBuf;

    use super::check;
    use crate::rules::no_dunder_tests::NoDunderTests;
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
            "discovery/__basic__/newtypes.rs",
            "#[cfg(test)] mod tests { #[test] fn test_usage() {} }",
        )]);
        let violations = check(&NoDunderTests, &project);
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message.contains("__basic__"));
    }

    #[test]
    fn test_usage_allows_tests_outside_dunder() {
        let project = project(&[(
            "discovery/newtypes.rs",
            "#[cfg(test)] mod tests { #[test] fn test_usage() {} }",
        )]);
        assert!(check(&NoDunderTests, &project).is_empty());
    }
}

// no test_usage necessary
