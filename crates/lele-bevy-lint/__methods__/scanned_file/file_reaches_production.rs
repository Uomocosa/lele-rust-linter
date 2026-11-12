use crate::scan;

pub fn file_reaches_production(file: &scan::ScannedFile, graph: &scan::CallGraph) -> bool {
    file.items.iter().any(|item| match item {
        syn::Item::Fn(func) => graph.reaches_production(&func.sig.ident.to_string()),
        syn::Item::Impl(impl_block) => impl_block.items.iter().any(|item| {
            matches!(item, syn::ImplItem::Fn(method) if graph.reaches_production(&method.sig.ident.to_string()))
        }),
        _ => false,
    })
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::file_reaches_production;
    use crate::scan;

    #[test]
    fn test_usage() {
        let file: syn::File = syn::parse_quote! {
            pub fn build() { helper(); }
            fn helper() {}
        };
        let mut project = lele_lint::Project::default();
        project
            .parsed_files
            .insert(PathBuf::from("a.rs"), file.clone());
        let graph = scan::CallGraph::from(&project);
        assert!(file_reaches_production(&scan::ScannedFile(&file), &graph));
        let orphan: syn::File = syn::parse_quote! { fn orphan() {} };
        assert!(!file_reaches_production(
            &scan::ScannedFile(&orphan),
            &graph
        ));
    }
}
