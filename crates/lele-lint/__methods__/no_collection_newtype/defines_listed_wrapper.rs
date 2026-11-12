use crate::rules;
use crate::CollectionWrapperEntry;
use crate::Project;

pub fn defines_listed_wrapper(project: &Project, entry: &CollectionWrapperEntry) -> bool {
    let path = project.root.join(&entry.file);
    project.content_sources().any(|source| {
        project.absolute_path(source.origin, source.relative_path) == path
            && source.file.items.iter().any(|item| {
                matches!(item, syn::Item::Struct(strukt) if strukt.ident == entry.type_name
                    && rules::no_collection_newtype::NoCollectionNewtype::wraps_collection(strukt))
            })
    })
}

#[cfg(test)]
mod tests {
    use crate::rules;
    use crate::CollectionWrapperEntry;
    use crate::Project;
    use std::path::PathBuf;

    #[test]
    fn test_usage() {
        let mut project = Project {
            root: PathBuf::from("/crate"),
            src_dir: PathBuf::from("/crate/src"),
            ..Project::default()
        };
        project.parsed_files.insert(
            PathBuf::from("cards/deck.rs"),
            syn::parse_str("pub struct Deck(Vec<Card>);").unwrap(),
        );
        let entry = CollectionWrapperEntry {
            type_name: "Deck".to_string(),
            file: PathBuf::from("src/cards/deck.rs"),
            reason: "cap".to_string(),
        };
        assert!(
            rules::no_collection_newtype::NoCollectionNewtype::defines_listed_wrapper(
                &project, &entry
            )
        );
    }
}
