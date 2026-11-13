use crate::rules;
use crate::Project;

pub fn without_listed_methods(project: &Project, impl_block: &syn::ItemImpl) -> syn::ItemImpl {
    let listed =
        rules::inline_trait_methods::InlineTraitMethods::listed_methods(project, impl_block);
    let mut owned = impl_block.clone();
    owned.items.retain(|item| match item {
        syn::ImplItem::Fn(method) => !listed.contains(&method.sig.ident.to_string()),
        _ => true,
    });
    owned
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
                trait_path: "Plugin".to_string(),
                methods: vec!["build".to_string()],
                reason: "framework reads the impl".to_string(),
            }],
            ..Project::default()
        };
        let plugin: syn::ItemImpl =
            syn::parse_quote! { impl Plugin for App { fn build(&self) {} fn name(&self) {} } };
        let owned = rules::inline_trait_methods::InlineTraitMethods::without_listed_methods(
            &project, &plugin,
        );
        assert_eq!(owned.items.len(), 1);
    }
}
