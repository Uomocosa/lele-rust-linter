use lele_lint::Project;
use lele_lint::RuleViolation;
use syn::spanned::Spanned;

use crate::rules;
use crate::scan;

pub fn rule_check(
    _self: &rules::no_tests_in_systems::NoTestsInSystems,
    project: &Project,
) -> Vec<RuleViolation> {
    let mut violations = Vec::new();
    for source in project.content_sources() {
        let in_systems = source
            .relative_path
            .components()
            .any(|c| c.as_os_str() == "__bevy_systems__" || c.as_os_str() == "bevy_systems");
        if !in_systems {
            continue;
        }
        let mut stack: Vec<&syn::Item> = source.file.items.iter().collect();
        while let Some(item) = stack.pop() {
            if let Some(module) =
                rules::no_tests_in_systems::NoTestsInSystems::cfg_test_module(item)
                && let Some((_, inner)) = &module.content
            {
                let previews = rules::no_tests_in_systems::NoTestsInSystems::preview_names(inner);
                for sub in inner {
                    if rules::no_tests_in_systems::NoTestsInSystems::is_allowed_preview_item(
                        sub, &previews,
                    ) {
                        continue;
                    }
                    let name = match sub {
                        syn::Item::Fn(func) => func.sig.ident.to_string(),
                        syn::Item::Mod(module) => module.ident.to_string(),
                        syn::Item::Struct(found) => found.ident.to_string(),
                        syn::Item::Enum(found) => found.ident.to_string(),
                        _ => "item".to_string(),
                    };
                    violations.push(scan::BevyProject(project).violation(
                        source.origin,
                        source.relative_path,
                        sub.span().start().line,
                        rules::no_tests_in_systems::NoTestsInSystems::CODE,
                        format!(
                            "`{name}` in `#[cfg(test)]` is not a UI preview — delete it or move the test to the owning `*_plugin.rs` tests; only `*_ui_*_preview` tests (plus `use`/`const`) may stay in a `__bevy_systems__/` file",
                        ),
                    ));
                }
                continue;
            }
            if let Some(line) = rules::no_tests_in_systems::NoTestsInSystems::cfg_test_line(item) {
                violations.push(scan::BevyProject(project).violation(
                    source.origin,
                    source.relative_path,
                    line,
                    rules::no_tests_in_systems::NoTestsInSystems::CODE,
                    "`#[cfg(test)]` does not belong in a `__bevy_systems__/` file — delete the test module and test the core where it lives, or the wiring in the owning `*_plugin.rs` tests".to_string(),
                ));
                continue;
            }
            if let syn::Item::Mod(module) = item
                && let Some((_, inner)) = &module.content
            {
                stack.extend(inner.iter());
            }
        }
    }
    violations
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use lele_lint::Project;

    use super::rule_check;
    use crate::rules;

    fn project_with(sources: &[(&str, &str)]) -> Project {
        let mut project = Project::default();
        for (path, code) in sources {
            let file: syn::File = syn::parse_str(code).unwrap();
            project.parsed_files.insert(PathBuf::from(path), file);
        }
        project
    }

    fn violations(source: &str) -> Vec<lele_lint::RuleViolation> {
        let project = project_with(&[("enemy/__bevy_systems__/tick_enemies.rs", source)]);
        rule_check(&rules::no_tests_in_systems::NoTestsInSystems, &project)
    }

    #[test]
    fn test_usage() {
        let found = violations(
            "pub struct Query;\n\npub fn tick_enemies(_q: Query) {}\n\n#[cfg(test)]\nmod tests {\n    #[test]\n    fn test_usage() {}\n}\n",
        );
        assert_eq!(found.len(), 1);
        assert_eq!(
            found[0].code,
            rules::no_tests_in_systems::NoTestsInSystems::CODE
        );
        assert!(found[0].message.contains("__bevy_systems__"));
    }

    #[test]
    fn test_usage_empty_test_module_is_clean() {
        let found = violations("#[cfg(test)]\nmod tests {}\n");
        assert!(found.is_empty());
    }

    #[test]
    fn test_usage_literal_bevy_systems_folder_is_flagged() {
        let project = project_with(&[(
            "enemy/bevy_systems/despawn.rs",
            "pub fn despawn() {}\n\n#[cfg(test)]\nmod tests {\n    #[test]\n    fn regression() {}\n}\n",
        )]);
        let found = rule_check(&rules::no_tests_in_systems::NoTestsInSystems, &project);
        assert_eq!(found.len(), 1);
        assert!(found[0].message.contains("regression"));
    }

    #[test]
    fn test_usage_cfg_test_fn_is_flagged() {
        let found = violations("pub fn tick_enemies() {}\n\n#[cfg(test)]\nfn helper() {}\n");
        assert_eq!(found.len(), 1);
    }

    #[test]
    fn test_usage_nested_cfg_test_is_flagged() {
        let found = violations("pub mod inner {\n    #[cfg(test)]\n    fn helper() {}\n}\n");
        assert_eq!(found.len(), 1);
    }

    #[test]
    fn test_usage_preview_only_module_is_clean() {
        let found = violations(
            "pub struct Query;\n\npub fn tick_enemies(_q: Query) {}\n\n#[cfg(test)]\nmod tests {\n    use super::Query;\n    use super::tick_enemies;\n    #[test]\n    fn tick_enemies_ui_png_preview() {\n        tick_enemies(Query);\n    }\n}\n",
        );
        assert!(found.is_empty());
    }

    #[test]
    fn test_usage_preview_plus_plain_test_is_flagged_once() {
        let found = violations(
            "pub struct Query;\n\npub fn tick_enemies(_q: Query) {}\n\n#[cfg(test)]\nmod tests {\n    use super::Query;\n    #[test]\n    fn tick_enemies_ui_png_preview() {}\n    #[test]\n    fn extra_regression() {}\n}\n",
        );
        assert_eq!(found.len(), 1);
        assert!(found[0].message.contains("extra_regression"));
    }

    #[test]
    fn test_usage_preview_plus_helper_is_flagged() {
        let found = violations(
            "pub struct Query;\n\npub fn tick_enemies(_q: Query) {}\n\n#[cfg(test)]\nmod tests {\n    fn preview_build() {}\n    #[test]\n    fn tick_enemies_ui_png_preview() {}\n}\n",
        );
        assert_eq!(found.len(), 1);
        assert!(found[0].message.contains("preview_build"));
    }

    #[test]
    fn test_usage_systems_file_without_tests_is_clean() {
        let found = violations("pub struct Query;\n\npub fn tick_enemies(_q: Query) {}\n");
        assert!(found.is_empty());
    }

    #[test]
    fn test_usage_tests_outside_systems_dir_are_clean() {
        let project = project_with(&[(
            "enemy/tick_enemies.rs",
            "pub fn tick_enemies() {}\n\n#[cfg(test)]\nmod tests {}\n",
        )]);
        assert!(rule_check(&rules::no_tests_in_systems::NoTestsInSystems, &project).is_empty());
    }
}
