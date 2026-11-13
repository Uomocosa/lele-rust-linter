use crate::CollectionWrapperEntry;
use crate::Project;
use std::path::Path;

pub fn listed_entry(
    project: &Project,
    struct_def: &syn::ItemStruct,
    file_path: &Path,
) -> Option<CollectionWrapperEntry> {
    project
        .collection_wrappers
        .iter()
        .find(|entry| {
            struct_def.ident == entry.type_name
                && Project::same_file(&project.config_path(&entry.file), file_path)
        })
        .cloned()
}

#[cfg(test)]
mod tests {
    use crate::rules;
    use crate::CollectionWrapperEntry;
    use crate::Project;
    use std::path::Path;
    use std::path::PathBuf;

    #[test]
    fn test_usage() {
        let project = Project {
            root: PathBuf::from("/repo"),
            collection_wrappers: vec![CollectionWrapperEntry {
                type_name: "Deck".to_string(),
                file: PathBuf::from("src/deck.rs"),
                reason: "at most 52 cards".to_string(),
            }],
            ..Project::default()
        };
        let deck: syn::ItemStruct = syn::parse_quote! { pub struct Deck(Vec<u8>); };
        assert!(
            rules::no_collection_newtype::NoCollectionNewtype::listed_entry(
                &project,
                &deck,
                Path::new("/repo/src/deck.rs")
            )
            .is_some()
        );
        assert!(
            rules::no_collection_newtype::NoCollectionNewtype::listed_entry(
                &project,
                &deck,
                Path::new("/repo/src/other.rs")
            )
            .is_none()
        );
    }
}
