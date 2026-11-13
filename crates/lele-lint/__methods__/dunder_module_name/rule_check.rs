use syn::visit::Visit;

use crate::rules;
use crate::Project;
use crate::RuleViolation;

pub fn rule_check(
    _self: &rules::dunder_module_name::DunderModuleName,
    project: &Project,
) -> Vec<RuleViolation> {
    let mut violations = Vec::new();

    for source in project.content_sources() {
        let mut hits = Vec::new();
        let mut reexports = Vec::new();
        {
            let mut visitor = rules::dunder_module_name::DunderNameVisitor {
                dunders: &project.dunder_paths,
                hits: &mut hits,
                reexports: &mut reexports,
                domain_prefix: rules::dunder_module_name::DunderModuleName::mount_prefix(
                    source.relative_path,
                    project,
                ),
                methods_mount_name: project.methods_mount_name(),
            };
            visitor.visit_file(source.file);
        }
        for hit in hits {
            violations.push(RuleViolation::from_site(
                project.absolute_path(source.origin, source.relative_path),
                hit.line,
                0,
                rules::dunder_module_name::DunderModuleName::CODE,
                hit.message,
            ));
        }
        for hit in reexports {
            if hit.mount == rules::dunder_module_name::DunderModuleName::BEVY_OWNED_MOUNT {
                continue;
            }
            violations.push(RuleViolation::from_site(
                project.absolute_path(source.origin, source.relative_path),
                hit.line,
                0,
                rules::dunder_module_name::DunderModuleName::CODE,
                rules::dunder_module_name::DunderModuleName::reexport_message(
                    &hit.item,
                    &hit.suggestion,
                ),
            ));
        }
    }

    violations
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::rule_check;
    use crate::rules::dunder_module_name::DunderModuleName;
    use crate::DunderPath;
    use crate::Project;

    fn messages(src: &str) -> Vec<String> {
        let mut project = Project {
            dunder_paths: DunderPath::defaults(),
            ..Project::default()
        };
        project
            .parsed_files
            .insert(PathBuf::from("stock/mod.rs"), syn::parse_str(src).unwrap());
        rule_check(&DunderModuleName, &project)
            .into_iter()
            .map(|violation| violation.message)
            .collect()
    }

    fn messages_with_methods_root(methods_root: &str, src: &str) -> Vec<String> {
        let mut project = Project {
            dunder_paths: DunderPath::defaults(),
            methods_dir: Some(PathBuf::from(methods_root)),
            ..Project::default()
        };
        project
            .parsed_files
            .insert(PathBuf::from("stock/mod.rs"), syn::parse_str(src).unwrap());
        rule_check(&DunderModuleName, &project)
            .into_iter()
            .map(|violation| violation.message)
            .collect()
    }

    #[test]
    fn test_usage() {
        assert_eq!(
            messages("#[path = \"__basic__/mod.rs\"]\nmod basic;\n"),
            Vec::<String>::new()
        );
        let found = messages("#[path = \"__basic__/mod.rs\"]\npub mod __basic__;\n");
        assert_eq!(found.len(), 1);
        assert!(found[0].contains("pub mod basic;"));
    }

    #[test]
    fn test_usage_flags_dunder_mod_without_path() {
        let found = messages("pub mod __basic__;\n");
        assert_eq!(found.len(), 1);
        assert!(found[0].contains("#[path = \"__basic__/mod.rs\"]"));
    }

    #[test]
    fn test_usage_flags_path_to_dunder_with_other_name() {
        let found = messages("#[path = \"__basic__/mod.rs\"]\npub mod helpers;\n");
        assert_eq!(found.len(), 1);
        assert!(found[0].contains("name the module `basic`, not `helpers`"));
    }

    #[test]
    fn test_usage_allows_path_to_file_inside_dunder() {
        assert_eq!(
            messages("#[path = \"__basic__/enums.rs\"]\nmod enums;\n"),
            Vec::<String>::new()
        );
    }

    #[test]
    fn test_usage_flags_dunder_path_segments() {
        let src = "use crate::__basic__;\nuse crate::stock::__basic__::enums;\n\
                   fn f(level: __basic__::enums::Bump) {}\n";
        assert_eq!(messages(src).len(), 3);
    }

    #[test]
    fn test_usage_ignores_unconfigured_dunders() {
        assert_eq!(
            messages("pub mod __other__;\nuse crate::__other__;\n"),
            Vec::<String>::new()
        );
    }

    #[test]
    fn test_usage_flags_pub_basic_mount() {
        let found = messages(
            "#[path = \"__basic__/mod.rs\"]\npub mod basic;\n\npub use basic::structs::Item;\n",
        );
        assert_eq!(found.len(), 1);
        assert_eq!(
            found[0],
            "`pub mod basic;` must be private: write `mod basic;` and re-export each item flat (`pub use basic::enums::HelloKind;`)"
        );
    }

    #[test]
    fn test_usage_allows_private_basic_mount_with_flat_reexport() {
        assert_eq!(
            messages(
                "#[path = \"__basic__/mod.rs\"]\nmod basic;\n\npub use basic::structs::Item;\n"
            ),
            Vec::<String>::new()
        );
    }

    #[test]
    fn test_usage_flags_private_traits_mount() {
        let found = messages("#[path = \"__traits__/mod.rs\"]\nmod traits;\n");
        assert_eq!(found.len(), 1);
        assert!(found[0].contains("declare `pub mod traits;`"));
    }

    #[test]
    fn test_usage_flags_trait_reexports() {
        let found = messages(
            "#[path = \"__traits__/mod.rs\"]\npub mod traits;\n\npub use traits::Greetable;\n",
        );
        assert_eq!(found.len(), 1);
        assert_eq!(
            found[0],
            "do not re-export `Greetable`: refer to it as `stock::traits::Greetable`"
        );
    }

    #[test]
    fn test_usage_flags_pub_crate_trait_reexports() {
        let found = messages(
            "#[path = \"__traits__/mod.rs\"]\npub mod traits;\n\npub(crate) use traits::Greetable;\n",
        );
        assert_eq!(found.len(), 1);
    }

    #[test]
    fn test_usage_ignores_private_trait_imports() {
        assert_eq!(
            messages(
                "#[path = \"__traits__/mod.rs\"]\npub mod traits;\n\nuse traits::Greetable;\n"
            ),
            Vec::<String>::new()
        );
    }

    #[test]
    fn test_usage_leaves_bevy_systems_reexports_to_bevy_export() {
        assert_eq!(
            messages(
                "#[path = \"__bevy_systems__/mod.rs\"]\npub mod bevy_systems;\n\npub use bevy_systems::poll_inv;\n",
            ),
            Vec::<String>::new()
        );
    }

    #[test]
    fn test_usage_methods_mount_visibility_follows_the_configured_root() {
        let found = messages_with_methods_root(
            "bodies",
            "#[path = \"../bodies/mod.rs\"]\npub mod bodies;\n\npub mod methods;\n",
        );
        assert_eq!(found.len(), 1);
        assert_eq!(
            found[0],
            "`pub mod bodies;` must be private: write `mod bodies;` — delegate bodies are reached through their type's methods"
        );
    }
}

// no test_usage necessary
