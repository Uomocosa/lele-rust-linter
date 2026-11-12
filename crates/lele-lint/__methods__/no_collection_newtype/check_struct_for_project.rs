use crate::rules;
use crate::Project;
use crate::RuleViolation;

pub fn check_struct_for_project(
    struct_def: &syn::ItemStruct,
    file_path: &std::path::Path,
    project: &Project,
    items: &[syn::Item],
) -> Vec<RuleViolation> {
    match rules::no_collection_newtype::NoCollectionNewtype::listed_entry(
        project, struct_def, file_path,
    ) {
        Some(_) => rules::no_collection_newtype::NoCollectionNewtype::check_listed_wrapper(
            struct_def, file_path, items,
        ),
        None => {
            rules::no_collection_newtype::NoCollectionNewtype::check_struct(struct_def, file_path)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::rules;
    use crate::Project;
    use std::path::Path;

    #[test]
    fn test_usage() {
        let file = syn::parse_str::<syn::File>("pub struct Bag(pub Vec<u8>);").unwrap();
        let project = Project::default();
        let found = rules::no_collection_newtype::NoCollectionNewtype::check_struct_for_project(
            &syn::parse_quote!(
                pub struct Hand(pub Vec<String>);
            ),
            Path::new("x.rs"),
            &project,
            &file.items,
        );
        assert_eq!(found.len(), 1);
    }
}
