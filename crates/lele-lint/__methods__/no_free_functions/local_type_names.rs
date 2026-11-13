use crate::Project;
use std::collections::BTreeSet;

pub fn local_type_names(project: &Project) -> BTreeSet<String> {
    project
        .parsed_files
        .values()
        .flat_map(|file| file.items.iter())
        .filter_map(|item| match item {
            syn::Item::Struct(item) => Some(item.ident.to_string()),
            syn::Item::Enum(item) => Some(item.ident.to_string()),
            syn::Item::Trait(item) => Some(item.ident.to_string()),
            _ => None,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::rules;
    use crate::Project;
    use std::collections::BTreeSet;
    use std::path::PathBuf;

    #[test]
    fn test_usage() {
        let mut project = Project::default();
        project.parsed_files.insert(
            PathBuf::from("stock.rs"),
            syn::parse_quote! { pub struct Deck; pub enum Hand { A } pub trait Shuffle {} fn helper() {} },
        );
        assert_eq!(
            rules::no_free_functions::NoFreeFunctions::local_type_names(&project),
            BTreeSet::from([
                "Deck".to_string(),
                "Hand".to_string(),
                "Shuffle".to_string()
            ])
        );
    }
}
