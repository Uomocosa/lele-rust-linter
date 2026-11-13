use crate::rules;

pub fn collect_use_segments(tree: &syn::UseTree) -> Vec<String> {
    match tree {
        syn::UseTree::Path(p) => {
            let mut segs = vec![p.ident.to_string()];
            segs.extend(rules::domain_import::DomainImport::collect_use_segments(
                &p.tree,
            ));
            segs
        }
        syn::UseTree::Name(n) => vec![n.ident.to_string()],
        syn::UseTree::Rename(r) => vec![r.ident.to_string()],
        syn::UseTree::Glob(_) => Vec::new(),
        syn::UseTree::Group(_) => Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use crate::rules;

    #[test]
    fn test_usage() {
        let renamed: syn::ItemUse = syn::parse_quote! { use crate::stock::Item as Thing; };
        assert_eq!(
            rules::domain_import::DomainImport::collect_use_segments(&renamed.tree),
            ["crate", "stock", "Item"]
        );
        let grouped: syn::ItemUse = syn::parse_quote! { use crate::{stock, roster}; };
        assert_eq!(
            rules::domain_import::DomainImport::collect_use_segments(&grouped.tree),
            ["crate"]
        );
    }
}
