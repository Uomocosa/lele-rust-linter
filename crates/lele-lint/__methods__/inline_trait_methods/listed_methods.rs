use crate::rules;
use crate::Project;

pub fn listed_methods(project: &Project, impl_block: &syn::ItemImpl) -> Vec<String> {
    let Some((_, trait_path, _)) = impl_block.trait_.as_ref() else {
        return Vec::new();
    };
    project
        .inline_trait_methods
        .iter()
        .filter(|entry| {
            rules::inline_trait_methods::InlineTraitMethods::trait_path_matches(
                &entry.trait_path,
                trait_path,
            )
        })
        .flat_map(|entry| entry.methods.iter().cloned())
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::rules;
    use crate::InlineTraitMethodsEntry;
    use crate::Project;

    #[test]
    fn test_usage() {
        let project = Project {
            inline_trait_methods: vec![InlineTraitMethodsEntry {
                trait_path: "bevy::app::Plugin".to_string(),
                methods: vec!["build".to_string(), "ready".to_string()],
                reason: "framework reads the impl".to_string(),
            }],
            ..Project::default()
        };
        let plugin: syn::ItemImpl = syn::parse_quote! { impl Plugin for App {} };
        assert_eq!(
            rules::inline_trait_methods::InlineTraitMethods::listed_methods(&project, &plugin),
            vec!["build".to_string(), "ready".to_string()]
        );
        let inherent: syn::ItemImpl = syn::parse_quote! { impl App {} };
        assert_eq!(
            rules::inline_trait_methods::InlineTraitMethods::listed_methods(&project, &inherent),
            Vec::<String>::new()
        );
    }
}
