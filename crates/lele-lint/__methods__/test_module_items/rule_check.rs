use std::collections::BTreeSet;
use std::path::PathBuf;

use syn::spanned::Spanned;

use crate::rules;
use crate::DirRole;
use crate::Project;
use crate::RuleViolation;

pub fn rule_check(
    _self: &rules::test_module_items::TestModuleItems,
    project: &Project,
) -> Vec<RuleViolation> {
    let code = rules::test_module_items::TestModuleItems::CODE;
    let describe = |item: &syn::Item| match item {
        syn::Item::Fn(function) => format!("fn {}", function.sig.ident),
        syn::Item::Mod(module) => format!("mod {}", module.ident),
        syn::Item::Struct(item) => format!("struct {}", item.ident),
        syn::Item::Enum(item) => format!("enum {}", item.ident),
        syn::Item::Trait(item) => format!("trait {}", item.ident),
        syn::Item::Type(item) => format!("type {}", item.ident),
        syn::Item::Static(item) => format!("static {}", item.ident),
        syn::Item::Macro(_) => String::from("macro"),
        syn::Item::Impl(_) => String::from("impl"),
        _ => String::from("item"),
    };
    let mut out_of_line: BTreeSet<PathBuf> = BTreeSet::new();
    for source in project.content_sources() {
        if matches!(
            project.dir_role(source.origin, source.relative_path),
            DirRole::Tests
        ) {
            continue;
        }
        let candidates = rules::test_module_items::TestModuleItems::out_of_line_test_files(
            source.relative_path,
            source.file,
        );
        out_of_line.extend(
            candidates
                .into_iter()
                .filter(|candidate| project.parsed_files.contains_key(candidate)),
        );
    }

    let mut violations = Vec::new();
    for source in project.content_sources() {
        if matches!(
            project.dir_role(source.origin, source.relative_path),
            DirRole::Tests
        ) {
            continue;
        }
        let absolute = project.absolute_path(source.origin, source.relative_path);
        let violation = |item: &syn::Item| {
            RuleViolation::from_site(
                absolute.clone(),
                item.span().start().line,
                0,
                code,
                format!(
                    "`{}` is not a test — a `#[cfg(test)]` module holds only test fns, `use` and `const`; move it to `src/testing/__fixtures__/`",
                    describe(item)
                ),
            )
        };
        let is_out_of_line = out_of_line.contains(source.relative_path);
        if is_out_of_line {
            for item in &source.file.items {
                if rules::test_module_items::TestModuleItems::is_allowed(item) {
                    continue;
                }
                violations.push(violation(item));
            }
            continue;
        }
        for item in &source.file.items {
            if !rules::test_module_items::TestModuleItems::is_test_module(item) {
                continue;
            }
            let syn::Item::Mod(module) = item else {
                continue;
            };
            let Some((_, items)) = &module.content else {
                continue;
            };
            for inner in items {
                if rules::test_module_items::TestModuleItems::is_allowed(inner) {
                    continue;
                }
                violations.push(violation(inner));
            }
        }
    }

    violations
}

#[cfg(test)]
mod tests {
    use super::rule_check;
    use crate::rules::test_module_items::TestModuleItems;
    use crate::testing;
    use crate::Project;

    #[test]
    fn test_usage() {
        let project = testing::parsed_project(Project::default(), &[(
            "stock/guest.rs",
            "pub struct Guest(pub String);\n#[cfg(test)] mod tests { fn make_guest() -> Guest { Guest(String::new()) } #[test] fn test_usage() {} }",
        )]);
        let violations = rule_check(&TestModuleItems, &project);
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message.contains("fn make_guest"));
        assert!(violations[0].message.contains("src/testing/__fixtures__/"));
    }

    #[test]
    fn test_usage_allows_tests_use_and_const() {
        let project = testing::parsed_project(Project::default(), &[(
            "stock/guest.rs",
            "pub struct Guest(pub String);\n#[cfg(test)] mod tests { use crate::stock; const LIMIT: usize = 3; #[test] fn test_usage() { let _ = (stock::Guest(String::new()), LIMIT); } }",
        )]);
        assert!(rule_check(&TestModuleItems, &project).is_empty());
    }

    #[test]
    fn test_usage_reports_out_of_line_test_module_body() {
        let project = testing::parsed_project(Project::default(), &[
            ("run/mod.rs", "#[cfg(test)] mod tests;"),
            (
                "run/tests.rs",
                "fn make_guest() -> u8 { 1 }\n#[test] fn test_usage() { assert_eq!(make_guest(), 1); }",
            ),
        ]);
        let violations = rule_check(&TestModuleItems, &project);
        assert_eq!(violations.len(), 1);
        assert!(violations[0].message.contains("fn make_guest"));
    }

    #[test]
    fn test_usage_skips_tests_component_paths() {
        let project = testing::parsed_project(
            Project::default(),
            &[("e2e/tests/helpers.rs", "fn helper() -> u8 { 1 }")],
        );
        assert!(rule_check(&TestModuleItems, &project).is_empty());
    }
}
