use crate::ItemMarker;
use crate::Project;
use std::collections::HashSet;

pub fn own_error_enums(project: &Project) -> HashSet<String> {
    project
        .content_sources()
        .flat_map(|source| source.file.items.iter())
        .filter_map(|item| match item {
            syn::Item::Enum(item_enum)
                if item_enum
                    .attrs
                    .iter()
                    .any(|attr| ItemMarker(attr).derives_any_of(&["Error"])) =>
            {
                Some(item_enum.ident.to_string())
            }
            _ => None,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::rules;
    use crate::Project;
    use std::collections::HashSet;
    use std::path::PathBuf;

    #[test]
    fn test_usage() {
        let mut project = Project::default();
        project.parsed_files.insert(
            PathBuf::from("failure.rs"),
            syn::parse_quote! { #[derive(Error)] pub enum Failure { Bad } pub enum Plain { Ok } },
        );
        assert_eq!(
            rules::no_stringified_errors::NoStringifiedErrors::own_error_enums(&project),
            HashSet::from(["Failure".to_string()])
        );
    }
}
