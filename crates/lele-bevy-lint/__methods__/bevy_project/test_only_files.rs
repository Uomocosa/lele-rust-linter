use std::collections::HashSet;
use std::path::PathBuf;

use lele_lint::ItemMarker;

use crate::scan;

pub fn test_only_files(project: &scan::BevyProject) -> HashSet<PathBuf> {
    let mut found = HashSet::new();
    for source in project.sources() {
        for item in &source.file.items {
            let syn::Item::Mod(module) = item else {
                continue;
            };
            if module.content.is_some()
                || !module
                    .attrs
                    .iter()
                    .any(|attr| ItemMarker(attr).is_cfg_test())
            {
                continue;
            }
            found.extend(scan::BevyProject::module_files(
                source.relative_path,
                module,
            ));
        }
    }
    found
}

#[cfg(test)]
mod tests {
    use super::test_only_files;
    use crate::scan;
    use lele_lint::Project;
    use std::path::PathBuf;

    #[test]
    fn test_usage() {
        let mut project = Project::default();
        project.parsed_files.insert(
            PathBuf::from("lib.rs"),
            syn::parse_str(
                "#[cfg(test)] #[path = \"scene/tests.rs\"] mod scene_tests;
                 #[cfg(test)] mod helpers;
                 #[path = \"real.rs\"] mod real;",
            )
            .unwrap(),
        );
        project.parsed_files.insert(
            PathBuf::from("ui/row.rs"),
            syn::parse_str("#[cfg(test)] mod tests;").unwrap(),
        );
        let found = test_only_files(&scan::BevyProject(&project));
        assert!(found.contains(&PathBuf::from("scene/tests.rs")));
        assert!(found.contains(&PathBuf::from("helpers.rs")));
        assert!(found.contains(&PathBuf::from("ui/row/tests.rs")));
        assert!(!found.contains(&PathBuf::from("real.rs")));
    }
}
