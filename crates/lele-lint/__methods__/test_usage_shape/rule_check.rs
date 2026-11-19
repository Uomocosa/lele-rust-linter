use std::collections::BTreeSet;
use std::path::PathBuf;

use crate::rules;
use crate::Origin;
use crate::Project;
use crate::RuleViolation;

pub fn rule_check(
    _self: &rules::test_usage_shape::TestUsageShape,
    project: &Project,
) -> Vec<RuleViolation> {
    let code = rules::test_usage_shape::TestUsageShape::CODE;

    let mut out_of_line: BTreeSet<PathBuf> = BTreeSet::new();
    for source in project.content_sources() {
        for candidate in rules::test_module_items::TestModuleItems::out_of_line_test_files(
            source.relative_path,
            source.file,
        ) {
            if project.parsed_files.contains_key(&candidate)
                || project.methods_parsed_files.contains_key(&candidate)
            {
                out_of_line.insert(candidate);
            }
        }
    }

    let mut violations = Vec::new();
    for source in project.content_sources() {
        if out_of_line.contains(source.relative_path) {
            continue;
        }
        if rules::test_usage::TestUsage::needs_no_test_usage(source.relative_path, source.file) {
            continue;
        }
        if rules::test_usage::TestUsage::is_exempt(source.relative_path, source.file, project) {
            continue;
        }
        if matches!(source.origin, Origin::Methods)
            && rules::methods_layout::MethodsLayout::is_index_file(source.relative_path)
        {
            continue;
        }
        let absolute = project.absolute_path(source.origin, source.relative_path);
        if rules::test_usage::TestUsage::reads_opt_out(&absolute) {
            continue;
        }

        let mut usage_fns = rules::test_usage_shape::TestUsageShape::test_usage_fns(source.file);
        for candidate in rules::test_module_items::TestModuleItems::out_of_line_test_files(
            source.relative_path,
            source.file,
        ) {
            let resolved = match source.origin {
                Origin::Methods => project.methods_parsed_files.get(&candidate),
                _ => project.parsed_files.get(&candidate),
            };
            if let Some(file) = resolved {
                usage_fns.extend(rules::test_usage_shape::TestUsageShape::test_usage_fns(
                    file,
                ));
            }
        }

        if usage_fns.is_empty() {
            continue;
        }
        if usage_fns.len() > 1 {
            violations.push(RuleViolation::from_file(
                absolute.clone(),
                code,
                format!(
                    "file `{}` has {} test_usage tests — keep exactly one and rename the others for their corner case",
                    source.relative_path.display(),
                    usage_fns.len()
                ),
            ));
        }

        let Some(primary) = rules::test_usage_shape::TestUsageShape::primary_item(
            source.file,
            source.relative_path,
            source.origin,
        ) else {
            continue;
        };
        let calls: Vec<rules::test_usage_shape::TestUsageCalls> = usage_fns
            .iter()
            .map(|function| rules::test_usage_shape::TestUsageCalls::from(*function))
            .collect();

        let calls_primary = calls
            .iter()
            .any(|call| rules::test_usage_shape::TestUsageShape::calls_primary(call, &primary));
        if !calls_primary {
            violations.push(RuleViolation::from_file(
                absolute.clone(),
                code,
                format!("test_usage never calls {}", primary.name),
            ));
        }

        let asserts_derived_value = usage_fns.iter().zip(calls.iter()).any(|(function, call)| {
            rules::test_usage_shape::TestUsageShape::asserts_derived_value(function, call, &primary)
        });
        if !asserts_derived_value {
            violations.push(RuleViolation::from_file(
                absolute,
                code,
                format!(
                    "test_usage has no assert on a value {} produced",
                    primary.name
                ),
            ));
        }
    }

    violations
}

#[cfg(test)]
mod tests {
    use super::rule_check;
    use crate::rules;
    use crate::testing;
    use crate::Project;
    use std::path::PathBuf;

    const GUEST: &str = "use derive_more::Deref;\n\n#[derive(Deref)]\npub struct Guest(pub String);\n\n#[rustfmt::skip]\nimpl Guest {\n    pub fn greet(&self) -> String { format!(\"Hello, {}!\", **self) }\n}\n";

    #[test]
    fn test_usage() {
        let project = testing::parsed_project(Project::default(), &[(
            "guest.rs",
            &(GUEST.to_string()
                + "\n#[cfg(test)]\nmod tests {\n    use super::Guest;\n\n    #[test]\n    fn test_usage() {\n        let guest = Guest(\"Ada\".to_string());\n        assert_eq!(guest.greet(), \"Hello, Ada!\");\n    }\n}\n"),
        )]);
        assert!(rule_check(&rules::test_usage_shape::TestUsageShape, &project).is_empty());
    }

    #[test]
    fn test_usage_reports_several() {
        let project = testing::parsed_project(Project::default(), &[(
            "guest.rs",
            &(GUEST.to_string()
                + "\n#[cfg(test)]\nmod tests {\n    use super::Guest;\n\n    #[test]\n    fn test_usage() {\n        let guest = Guest(\"Ada\".to_string());\n        assert_eq!(guest.greet(), \"Hello, Ada!\");\n    }\n}\n\n#[cfg(test)]\nmod more {\n    use super::Guest;\n\n    #[test]\n    fn test_usage() {\n        let guest = Guest(\"Bob\".to_string());\n        assert_eq!(guest.greet(), \"Hello, Bob!\");\n    }\n}\n"),
        )]);
        let violations = rule_check(&rules::test_usage_shape::TestUsageShape, &project);
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message.contains("has 2 test_usage tests"));
    }

    #[test]
    fn test_usage_reports_a_test_usage_that_never_calls_the_type() {
        let project = testing::parsed_project(Project::default(), &[(
            "guest.rs",
            &(GUEST.to_string()
                + "\n#[cfg(test)]\nmod tests {\n    #[test]\n    fn test_usage() {\n        assert!(true);\n    }\n}\n"),
        )]);
        let violations = rule_check(&rules::test_usage_shape::TestUsageShape, &project);
        assert!(violations
            .iter()
            .any(|violation| violation.message == "test_usage never calls Guest"));
    }

    #[test]
    fn test_usage_reports_a_test_usage_that_never_calls_the_fn() {
        let project = testing::parsed_project(Project::default(), &[(
            "compute.rs",
            "pub fn compute(value: u32) -> u32 {\n    value * 2\n}\n\n#[cfg(test)]\nmod tests {\n    #[test]\n    fn test_usage() {\n        assert_eq!(1 + 1, 2);\n    }\n}\n",
        )]);
        let violations = rule_check(&rules::test_usage_shape::TestUsageShape, &project);
        assert!(violations
            .iter()
            .any(|violation| violation.message == "test_usage never calls compute"));
    }

    #[test]
    fn test_usage_reports_a_method_test_usage_that_never_calls_the_method() {
        let mut project = Project::default();
        project.methods_parsed_files.insert(
            PathBuf::from("counter/increment.rs"),
            syn::parse_str(
                "pub fn increment(counter: &mut u8) {\n    *counter += 1;\n}\n\n#[cfg(test)]\nmod tests {\n    #[test]\n    fn test_usage() {\n        assert_eq!(1 + 1, 2);\n    }\n}\n",
            )
            .unwrap(),
        );
        let violations = rule_check(&rules::test_usage_shape::TestUsageShape, &project);
        assert!(violations
            .iter()
            .any(|violation| violation.message == "test_usage never calls increment"));
    }

    #[test]
    fn test_usage_reports_a_plugin_test_usage_that_never_installs_the_plugin() {
        let project = testing::parsed_project(Project::default(), &[(
            "p2p_plugin.rs",
            "pub struct P2PPlugin;\n\nimpl P2PPlugin {\n    pub fn build(&self) -> u32 {\n        1\n    }\n}\n\n#[cfg(test)]\nmod tests {\n    #[test]\n    fn test_usage() {\n        assert!(true);\n    }\n}\n",
        )]);
        let violations = rule_check(&rules::test_usage_shape::TestUsageShape, &project);
        assert!(violations
            .iter()
            .any(|violation| violation.message == "test_usage never calls P2PPlugin"));
    }

    #[test]
    fn test_usage_reports_a_test_usage_without_an_assert() {
        let project = testing::parsed_project(Project::default(), &[(
            "guest.rs",
            &(GUEST.to_string()
                + "\n#[cfg(test)]\nmod tests {\n    use super::Guest;\n\n    #[test]\n    fn test_usage() {\n        let guest = Guest(\"Ada\".to_string());\n        let _ = guest.greet();\n    }\n}\n"),
        )]);
        let violations = rule_check(&rules::test_usage_shape::TestUsageShape, &project);
        assert!(violations
            .iter()
            .any(|violation| violation.message
                == "test_usage has no assert on a value Guest produced"));
    }

    #[test]
    fn test_usage_accepts_a_call_inside_the_assert_macro() {
        let project = testing::parsed_project(Project::default(), &[(
            "guest.rs",
            &(GUEST.to_string()
                + "\n#[cfg(test)]\nmod tests {\n    use super::Guest;\n\n    #[test]\n    fn test_usage() {\n        assert_eq!(Guest(\"Ada\".to_string()).greet(), \"Hello, Ada!\");\n    }\n}\n"),
        )]);
        assert!(rule_check(&rules::test_usage_shape::TestUsageShape, &project).is_empty());
    }

    #[test]
    fn test_usage_accepts_a_one_level_let_chain() {
        let project = testing::parsed_project(Project::default(), &[(
            "guest.rs",
            &(GUEST.to_string()
                + "\n#[cfg(test)]\nmod tests {\n    use super::Guest;\n\n    #[test]\n    fn test_usage() {\n        let guest = Guest(\"Ada\".to_string());\n        let greeting = guest.greet();\n        assert_eq!(greeting, \"Hello, Ada!\");\n    }\n}\n"),
        )]);
        assert!(rule_check(&rules::test_usage_shape::TestUsageShape, &project).is_empty());
    }

    #[test]
    fn test_usage_accepts_an_assert_on_the_receiver_of_a_unit_method() {
        let mut project = Project::default();
        project.methods_parsed_files.insert(
            PathBuf::from("counter/increment.rs"),
            syn::parse_str(
                "pub fn increment(counter: &mut u8) {\n    *counter += 1;\n}\n\n#[cfg(test)]\nmod tests {\n    use super::increment;\n\n    #[test]\n    fn test_usage() {\n        let mut counter = 0;\n        increment(&mut counter);\n        assert_eq!(counter, 1);\n    }\n}\n",
            )
            .unwrap(),
        );
        assert!(rule_check(&rules::test_usage_shape::TestUsageShape, &project).is_empty());
    }

    #[test]
    fn test_usage_accepts_a_bare_statement_call_asserted_on_its_receiver() {
        let mut project = Project::default();
        project.methods_parsed_files.insert(
            PathBuf::from("visitor/visit_path.rs"),
            syn::parse_str(
                "pub fn visit_path(visitor: &mut Visitor, path: &syn::Path) {\n    visitor.hits += path.segments.len();\n}\n\n#[cfg(test)]\nmod tests {\n    use super::visit_path;\n\n    #[test]\n    fn test_usage() {\n        let mut visitor = Visitor::default();\n        let path: syn::Path = syn::parse_str(\"a::b\").unwrap();\n        visit_path(&mut visitor, &path);\n        assert_eq!(visitor.hits, 2);\n    }\n}\n",
            )
            .unwrap(),
        );
        assert!(rule_check(&rules::test_usage_shape::TestUsageShape, &project).is_empty());
    }

    #[test]
    fn test_usage_skips_a_file_that_opts_out() {
        let dir = tempfile::tempdir().unwrap();
        let rel = std::path::Path::new("guest.rs");
        let absolute = dir.path().join(rel);
        let source = format!("{GUEST}\n// no test_usage necessary\n");
        std::fs::write(&absolute, &source).unwrap();
        let mut project = testing::parsed_project(Project::default(), &[("guest.rs", &source)]);
        project.src_dir = dir.path().to_path_buf();
        assert!(rule_check(&rules::test_usage_shape::TestUsageShape, &project).is_empty());
    }

    #[test]
    fn test_usage_skips_a_bevy_systems_file() {
        let project = testing::parsed_project(
            Project::default(),
            &[("enemy/__bevy_systems__/tick.rs", "pub fn tick() {}\n")],
        );
        assert!(rule_check(&rules::test_usage_shape::TestUsageShape, &project).is_empty());
    }

    #[test]
    fn test_usage_detects_methods_by_origin_not_by_path() {
        let mut project = Project::default();
        project.methods_parsed_files.insert(
            PathBuf::from("counter/increment.rs"),
            syn::parse_str(
                "pub fn increment(counter: &mut u8) {\n    *counter += 1;\n}\n\n#[cfg(test)]\nmod tests {\n    use super::increment;\n\n    #[test]\n    fn test_usage() {\n        let mut counter = 0;\n        counter.increment();\n        assert_eq!(counter, 1);\n    }\n}\n",
            )
            .unwrap(),
        );
        assert!(rule_check(&rules::test_usage_shape::TestUsageShape, &project).is_empty());
    }

    #[test]
    fn test_usage_reads_a_multi_root_method_path() {
        let mut project = Project {
            layout_roots: vec![PathBuf::from("src"), PathBuf::from("ui")],
            ..Project::default()
        };
        project.methods_parsed_files.insert(
            PathBuf::from("src/widget/spin.rs"),
            syn::parse_str(
                "pub fn spin(widget: &mut u8) {\n    *widget += 1;\n}\n\n#[cfg(test)]\nmod tests {\n    use super::spin;\n\n    #[test]\n    fn test_usage() {\n        let mut widget = 0;\n        spin(&mut widget);\n        assert_eq!(widget, 1);\n    }\n}\n",
            )
            .unwrap(),
        );
        assert!(rule_check(&rules::test_usage_shape::TestUsageShape, &project).is_empty());
    }

    #[test]
    fn test_usage_skips_an_impl_only_file() {
        let project = testing::parsed_project(Project::default(), &[(
            "output.rs",
            "impl Output {\n    pub fn render(&self) -> String {\n        String::new()\n    }\n}\n\n#[cfg(test)]\nmod tests {\n    #[test]\n    fn test_usage() {\n        assert!(true);\n    }\n}\n",
        )]);
        assert!(rule_check(&rules::test_usage_shape::TestUsageShape, &project).is_empty());
    }

    #[test]
    fn test_usage_accepts_a_long_test_usage() {
        let project = testing::parsed_project(Project::default(), &[(
            "guest.rs",
            &(GUEST.to_string()
                + "\n#[cfg(test)]\nmod tests {\n    use super::Guest;\n\n    #[test]\n    fn test_usage() {\n        let guest = Guest(\"Ada\".to_string());\n        assert_eq!(guest.greet(), \"Hello, Ada!\");\n        assert_eq!(guest.greet(), \"Hello, Ada!\");\n        assert_eq!(guest.greet(), \"Hello, Ada!\");\n        assert_eq!(guest.greet(), \"Hello, Ada!\");\n        assert_eq!(guest.greet(), \"Hello, Ada!\");\n        assert_eq!(guest.greet(), \"Hello, Ada!\");\n        assert_eq!(guest.greet(), \"Hello, Ada!\");\n        assert_eq!(guest.greet(), \"Hello, Ada!\");\n        assert_eq!(guest.greet(), \"Hello, Ada!\");\n        assert_eq!(guest.greet(), \"Hello, Ada!\");\n        assert_eq!(guest.greet(), \"Hello, Ada!\");\n        assert_eq!(guest.greet(), \"Hello, Ada!\");\n        assert_eq!(guest.greet(), \"Hello, Ada!\");\n        assert_eq!(guest.greet(), \"Hello, Ada!\");\n        assert_eq!(guest.greet(), \"Hello, Ada!\");\n        assert_eq!(guest.greet(), \"Hello, Ada!\");\n        assert_eq!(guest.greet(), \"Hello, Ada!\");\n        assert_eq!(guest.greet(), \"Hello, Ada!\");\n        assert_eq!(guest.greet(), \"Hello, Ada!\");\n        assert_eq!(guest.greet(), \"Hello, Ada!\");\n        assert_eq!(guest.greet(), \"Hello, Ada!\");\n        assert_eq!(guest.greet(), \"Hello, Ada!\");\n        assert_eq!(guest.greet(), \"Hello, Ada!\");\n        assert_eq!(guest.greet(), \"Hello, Ada!\");\n        assert_eq!(guest.greet(), \"Hello, Ada!\");\n        assert_eq!(guest.greet(), \"Hello, Ada!\");\n        assert_eq!(guest.greet(), \"Hello, Ada!\");\n        assert_eq!(guest.greet(), \"Hello, Ada!\");\n        assert_eq!(guest.greet(), \"Hello, Ada!\");\n        assert_eq!(guest.greet(), \"Hello, Ada!\");\n        assert_eq!(guest.greet(), \"Hello, Ada!\");\n        assert_eq!(guest.greet(), \"Hello, Ada!\");\n        assert_eq!(guest.greet(), \"Hello, Ada!\");\n        assert_eq!(guest.greet(), \"Hello, Ada!\");\n        assert_eq!(guest.greet(), \"Hello, Ada!\");\n        assert_eq!(guest.greet(), \"Hello, Ada!\");\n        assert_eq!(guest.greet(), \"Hello, Ada!\");\n        assert_eq!(guest.greet(), \"Hello, Ada!\");\n        assert_eq!(guest.greet(), \"Hello, Ada!\");\n        assert_eq!(guest.greet(), \"Hello, Ada!\");\n    }\n}\n"),
        )]);
        assert!(rule_check(&rules::test_usage_shape::TestUsageShape, &project).is_empty());
    }

    #[test]
    fn test_usage_counts_an_out_of_line_test_module_with_its_declaring_file() {
        let project = testing::parsed_project(Project::default(), &[
            (
                "run/widget.rs",
                "pub struct Widget;\n\nimpl Widget {\n    pub fn is_ready(&self) -> bool {\n        true\n    }\n}\n\n#[cfg(test)]\nmod tests;\n",
            ),
            (
                "run/tests.rs",
                "#[test]\nfn test_usage() {\n    assert!(true);\n}\n",
            ),
        ]);
        let violations = rule_check(&rules::test_usage_shape::TestUsageShape, &project);
        assert_eq!(violations.len(), 2);
        assert!(violations
            .iter()
            .all(|violation| violation.file.as_path() == std::path::Path::new("run/widget.rs")));
        assert!(violations
            .iter()
            .any(|violation| violation.message == "test_usage never calls Widget"));
    }
}
