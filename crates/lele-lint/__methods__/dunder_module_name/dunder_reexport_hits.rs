use std::path::Path;

use syn::visit::Visit;

use crate::rules;
use crate::Project;

pub fn dunder_reexport_hits(
    rel_path: &Path,
    file: &syn::File,
    project: &Project,
) -> Vec<rules::dunder_module_name::ReexportHit> {
    let mut hits = Vec::new();
    let mut reexports = Vec::new();
    let mut visitor = rules::dunder_module_name::DunderNameVisitor {
        dunders: &project.dunder_paths,
        hits: &mut hits,
        reexports: &mut reexports,
        domain_prefix: rules::dunder_module_name::DunderModuleName::mount_prefix(rel_path, project),
        methods_mount_name: project.methods_mount_name(),
    };
    visitor.visit_file(file);
    reexports
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::dunder_reexport_hits;
    use crate::DunderPath;
    use crate::Project;
    use std::path::Path;

    fn hits(rel_path: &str, source: &str) -> Vec<String> {
        let mut project = Project {
            dunder_paths: DunderPath::defaults(),
            ..Project::default()
        };
        project
            .parsed_files
            .insert(PathBuf::from(rel_path), syn::parse_str(source).unwrap());
        let file = project.get_parsed(Path::new(rel_path)).unwrap();
        dunder_reexport_hits(Path::new(rel_path), file, &project)
            .into_iter()
            .map(|hit| hit.suggestion)
            .collect()
    }

    #[test]
    fn test_usage() {
        let found = hits("greeting/mod.rs", "pub use traits::Greetable;\n");
        assert_eq!(found, vec!["greeting::traits::Greetable".to_string()]);
    }

    #[test]
    fn test_usage_flags_pub_crate_use_and_lib_rs() {
        let found = hits("lib.rs", "pub(crate) use stock::traits::Clock;\n");
        assert_eq!(found, vec!["stock::traits::Clock".to_string()]);
    }

    #[test]
    fn test_usage_ignores_private_use_and_basic_mounts() {
        let found = hits(
            "guest.rs",
            "use crate::stock::traits::Clock;\npub use basic::enums::Kind;\n",
        );
        assert_eq!(found, Vec::<String>::new());
    }

    #[test]
    fn test_usage_records_mount_item_and_raw() {
        let mut project = Project {
            dunder_paths: DunderPath::defaults(),
            ..Project::default()
        };
        project.parsed_files.insert(
            PathBuf::from("greeting/mod.rs"),
            syn::parse_str("pub use traits::Greetable;\n").unwrap(),
        );
        let file = project.get_parsed(Path::new("greeting/mod.rs")).unwrap();
        let hit = &dunder_reexport_hits(Path::new("greeting/mod.rs"), file, &project)[0];
        assert_eq!(hit.mount, "traits");
        assert_eq!(hit.item, "Greetable");
        assert_eq!(hit.raw, "traits::Greetable");
    }
}
