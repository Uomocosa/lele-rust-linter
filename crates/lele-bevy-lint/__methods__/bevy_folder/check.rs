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
                    "pub fn `{}` is registered with `app.add_systems()` but lives outside bevy_systems/ — move it into the domain's bevy_systems/ folder",
                    func.sig.ident
                ),
            ));
        }
    }

    violations
}

#[cfg(test)]
mod tests {
    use crate::rules;

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
}
