use crate::rules;

pub fn walk_use_tree(
    tree: &syn::UseTree,
    prefix: &mut Vec<String>,
    collector: &mut rules::boundary_imports::UseCollector,
    line: usize,
) {
    match tree {
        syn::UseTree::Path(path) => {
            let mut next = prefix.clone();
            next.push(path.ident.to_string());
            rules::boundary_imports::BoundaryImports::walk_use_tree(
                &path.tree, &mut next, collector, line,
            );
        }
        syn::UseTree::Name(name) => {
            let mut full = prefix.clone();
            if name.ident != "self" {
                full.push(name.ident.to_string());
            }
            let joined = full.join("::");
            collector
                .aliases
                .insert(name.ident.to_string(), joined.clone());
            collector.hits.push(rules::boundary_imports::PathHit {
                path: joined,
                line,
                glob: false,
            });
        }
        syn::UseTree::Rename(rename) => {
            let mut full = prefix.clone();
            full.push(rename.ident.to_string());
            let joined = full.join("::");
            collector
                .aliases
                .insert(rename.rename.to_string(), joined.clone());
            collector.hits.push(rules::boundary_imports::PathHit {
                path: joined,
                line,
                glob: false,
            });
        }
        syn::UseTree::Glob(_) => {
            collector.hits.push(rules::boundary_imports::PathHit {
                path: prefix.join("::"),
                line,
                glob: true,
            });
        }
        syn::UseTree::Group(group) => {
            for item in &group.items {
                rules::boundary_imports::BoundaryImports::walk_use_tree(
                    item, prefix, collector, line,
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::rules;
    use std::collections::HashMap;

    #[test]
    fn test_usage() {
        let item: syn::ItemUse = syn::parse_quote! { use std::{fmt::Display as Shown, io::*}; };
        let mut collector = rules::boundary_imports::UseCollector {
            aliases: HashMap::new(),
            hits: Vec::new(),
        };
        let mut prefix = Vec::new();
        rules::boundary_imports::BoundaryImports::walk_use_tree(
            &item.tree,
            &mut prefix,
            &mut collector,
            3,
        );
        assert_eq!(collector.hits.len(), 2);
        assert_eq!(collector.hits[0].path, "std::fmt::Display");
        assert!(!collector.hits[0].glob);
        assert_eq!(collector.hits[1].path, "std::io");
        assert!(collector.hits[1].glob);
        assert_eq!(
            collector.aliases.get("Shown").map(String::as_str),
            Some("std::fmt::Display")
        );
    }
}
