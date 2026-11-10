use std::collections::HashSet;

use syn::spanned::Spanned;

use lele_lint::Origin;
use lele_lint::Project;
use lele_lint::RuleViolation;

use crate::rules;
use crate::scan;

pub fn check(_self: &rules::bevy_folder::BevyFolder, project: &Project) -> Vec<RuleViolation> {
    let registered: HashSet<String> = project
        .sources()
        .flat_map(|source| rules::bevy_folder::BevyFolder::registered_systems(source.file))
        .collect();
    let mut violations = Vec::new();

    for source in project.sources() {
        if matches!(source.origin, Origin::Examples) {
            continue;
        }
        if source
            .relative_path
            .components()
            .any(|c| c.as_os_str().to_str() == Some("bevy_systems"))
        {
            violations.push(scan::BevyProject(project).violation(
                source.origin,
                source.relative_path,
                1,
                rules::bevy_folder::BevyFolder::CODE,
                "Bevy systems live in `__bevy_systems__/` — rename `bevy_systems/` to `__bevy_systems__/` and mount it with `#[path = \"__bevy_systems__/mod.rs\"] pub mod bevy_systems;`".to_string(),
            ));
            continue;
        }
        if source
            .relative_path
            .components()
            .any(|c| c.as_os_str().to_str() == Some("__bevy_systems__"))
        {
            continue;
        }

        for item in &source.file.items {
            let syn::Item::Fn(func) = item else {
                continue;
            };

            if !matches!(func.vis, syn::Visibility::Public(_)) {
                continue;
            }

            if !func
                .sig
                .inputs
                .iter()
                .any(rules::bevy_folder::BevyFolder::is_bevy_system_param)
            {
                continue;
            }

            let ident = func.sig.ident.to_string();
            if !registered.contains(&ident) {
                continue;
            }

            violations.push(scan::BevyProject(project).violation(source.origin, source.relative_path, func.sig.fn_token.span().start().line,
                rules::bevy_folder::BevyFolder::CODE,
                format!(
                    "pub fn `{}` is registered with `app.add_systems()` but lives outside `__bevy_systems__/` — move it into the domain's `__bevy_systems__/` folder",
                    func.sig.ident
                ),
            ));
        }
    }

    violations
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use lele_lint::DunderPath;
    use lele_lint::Project;

    use super::check;
    use crate::rules;

    fn project_with(sources: &[(&str, &str)]) -> Project {
        let mut project = Project {
            dunder_paths: DunderPath::defaults(),
            ..Project::default()
        };
        for (path, code) in sources {
            let file: syn::File = syn::parse_str(code).unwrap();
            project.parsed_files.insert(PathBuf::from(path), file);
        }
        project
    }

    #[test]
    fn test_usage() {
        let file: syn::File = syn::parse_str(
            "pub fn tick(mut q: Query<&Foo>, cmds: Commands) {}
             pub fn helper(x: u32) {}",
        )
        .unwrap();

        let syn::Item::Fn(system_fn) = &file.items[0] else {
            panic!("expected fn");
        };
        let syn::Item::Fn(plain_fn) = &file.items[1] else {
            panic!("expected fn");
        };

        assert!(
            system_fn
                .sig
                .inputs
                .iter()
                .any(rules::bevy_folder::BevyFolder::is_bevy_system_param)
        );
        assert!(
            !plain_fn
                .sig
                .inputs
                .iter()
                .any(rules::bevy_folder::BevyFolder::is_bevy_system_param)
        );

        let ty: syn::Type = syn::parse_str("&mut Query<&Foo>").unwrap();
        assert_eq!(
            rules::bevy_folder::BevyFolder::last_path_ident(&ty),
            Some("Query".to_string())
        );

        let scheduled: syn::File = syn::parse_str(
            "pub fn build(app: &mut App) {
                 app.add_systems(Update, tick);
                 app.add_systems(Startup, (setup, spawn).chain());
                 app.add_systems(Startup, chained).add_systems(Update, outer);
             }",
        )
        .unwrap();
        let registered = rules::bevy_folder::BevyFolder::registered_systems(&scheduled);
        assert!(registered.contains(&"chained".to_string()));
        assert!(registered.contains(&"outer".to_string()));
        assert!(registered.contains(&"tick".to_string()));
        assert!(registered.contains(&"setup".to_string()));
        assert!(registered.contains(&"spawn".to_string()));
        assert!(!registered.contains(&"Update".to_string()));
        assert!(!registered.contains(&"Startup".to_string()));

        let empty: syn::File = syn::parse_str("pub fn helper(x: u32) {}").unwrap();
        assert_eq!(
            rules::bevy_folder::BevyFolder::registered_systems(&empty).len(),
            0
        );
    }

    #[test]
    fn test_usage_ignores_test_module_registrations() {
        let file: syn::File = syn::parse_str(
            "pub fn tick(_q: Query) {}
             #[cfg(test)]
             mod tests {
                 fn helper(app: &mut App) { app.add_systems(Update, tick); }
             }",
        )
        .unwrap();
        assert_eq!(
            rules::bevy_folder::BevyFolder::registered_systems(&file).len(),
            0
        );
    }

    #[test]
    fn test_usage_reports_literal_bevy_systems_folder() {
        let project = project_with(&[(
            "enemy/bevy_systems/despawn.rs",
            "pub struct Query;\n\npub fn despawn(_q: Query) {}\n\npub fn register() {\n    let app = App;\n    app.add_systems(Update, despawn);\n}\n",
        )]);
        let violations = check(&rules::bevy_folder::BevyFolder, &project);
        assert_eq!(violations.len(), 1);
        assert!(
            violations[0]
                .message
                .contains("rename `bevy_systems/` to `__bevy_systems__/`")
        );
        assert_eq!(violations[0].code, rules::bevy_folder::BevyFolder::CODE);
    }

    #[test]
    fn test_usage_systems_files_in_dunder_folder_pass() {
        let project = project_with(&[(
            "enemy/__bevy_systems__/despawn.rs",
            "pub struct Query;\n\npub fn despawn(_q: Query) {}\n\npub fn register() {\n    let app = App;\n    app.add_systems(Update, despawn);\n}\n",
        )]);
        assert!(check(&rules::bevy_folder::BevyFolder, &project).is_empty());
    }

    #[test]
    fn test_usage_registered_system_outside_folder_flagged() {
        let project = project_with(&[(
            "enemy/tick_enemies.rs",
            "pub struct Query;\n\npub fn tick_enemies(_q: Query) {}\n\npub fn register() {\n    let app = App;\n    app.add_systems(Update, tick_enemies);\n}\n",
        )]);
        let violations = check(&rules::bevy_folder::BevyFolder, &project);
        assert_eq!(violations.len(), 1);
        assert!(
            violations[0]
                .message
                .contains("outside `__bevy_systems__/`")
        );
    }
}
