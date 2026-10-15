use std::collections::BTreeSet;

use lele_lint::Project;

use crate::VISUAL_IDENTS;

#[must_use]
pub fn require_visuals(project: &Project) -> BTreeSet<String> {
    let mut names = BTreeSet::new();
    for source in project.sources() {
        for item in &source.file.items {
            let syn::Item::Struct(item_struct) = item else {
                continue;
            };
            if requires_visual(&item_struct.attrs) {
                names.insert(item_struct.ident.to_string());
            }
        }
    }
    names
}

// needed helper: does a struct carry `#[require(<visual>)]`?
fn requires_visual(attrs: &[syn::Attribute]) -> bool {
    attrs
        .iter()
        .filter(|attr| attr.path().is_ident("require"))
        .any(|attr| {
            attr.parse_args_with(
                syn::punctuated::Punctuated::<syn::Path, syn::Token![,]>::parse_terminated,
            )
            .is_ok_and(|paths| {
                paths.iter().any(|path| {
                    path.segments.last().is_some_and(|segment| {
                        VISUAL_IDENTS.contains(&segment.ident.to_string().as_str())
                    })
                })
            })
        })
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::require_visuals;
    use lele_lint::Project;

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
        let names = require_visuals(&project);
        assert!(names.contains("Cell"));
        assert!(!names.contains("Plain"));
        assert!(!names.contains("Bare"));
    }
}
