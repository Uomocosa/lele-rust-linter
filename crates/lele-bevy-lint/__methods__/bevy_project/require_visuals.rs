use std::collections::BTreeSet;

use crate::scan;

pub fn require_visuals(project: &scan::BevyProject) -> BTreeSet<String> {
    let mut names = BTreeSet::new();
    for source in project.sources() {
        for item in &source.file.items {
            let syn::Item::Struct(item_struct) = item else {
                continue;
            };
            if scan::BevyProject::requires_visual(&item_struct.attrs) {
                names.insert(item_struct.ident.to_string());
            }
        }
    }
    names
}

#[cfg(test)]
mod tests {
    use super::require_visuals;
    use crate::scan;
    use lele_lint::Project;
    use std::path::PathBuf;

    #[test]
    fn test_usage() {
        let mut project = Project::default();
        project.parsed_files.insert(
            PathBuf::from("ui/components.rs"),
            syn::parse_str(
                "#[derive(Component)] #[require(Node)] pub struct Cell;
                 #[derive(Component)] #[require(Marker)] pub struct Plain;
                 #[derive(Component)] pub struct Bare;",
            )
            .unwrap(),
        );
        let names = require_visuals(&scan::BevyProject(&project));
        assert!(names.contains("Cell"));
        assert!(!names.contains("Plain"));
        assert!(!names.contains("Bare"));
    }
}
