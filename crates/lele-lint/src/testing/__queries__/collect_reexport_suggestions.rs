use std::path::Path;

use crate::methods;
use crate::rules;
use crate::testing;
use crate::Project;

pub(crate) fn collect_reexport_suggestions(rel_path: &str, source: &str) -> Vec<String> {
    let project: Project =
        testing::parsed_project(testing::dunder_project(), &[(rel_path, source)]);
    let file = project.get_parsed(Path::new(rel_path)).unwrap().clone();
    let Some(syn::Item::Use(item_use)) = file.items.first() else {
        return Vec::new();
    };
    let mut hits = Vec::new();
    let mut reexports = Vec::new();
    let mut visitor = rules::dunder_module_name::DunderNameVisitor {
        dunders: &project.dunder_paths,
        hits: &mut hits,
        reexports: &mut reexports,
        domain_prefix: methods::dunder_module_name::mount_prefix(Path::new(rel_path), &project),
        methods_mount_name: project.methods_mount_name(),
    };
    methods::dunder_module_name::collect_reexports(&mut visitor, item_use);
    reexports.iter().map(|hit| hit.suggestion.clone()).collect()
}

#[cfg(test)]
mod tests {
    use super::collect_reexport_suggestions;

    #[test]
    fn test_usage() {
        assert_eq!(
            collect_reexport_suggestions("greeting/mod.rs", "pub use traits::Greetable;\n"),
            vec!["greeting::traits::Greetable".to_string()]
        );
    }
}
