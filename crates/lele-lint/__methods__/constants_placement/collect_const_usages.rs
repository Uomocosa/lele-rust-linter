use crate::rules;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use syn::visit::Visit;

pub fn collect_const_usages(
    parsed_files: &HashMap<PathBuf, syn::File>,
    defined: &HashMap<String, Vec<PathBuf>>,
) -> HashMap<(String, PathBuf), HashSet<PathBuf>> {
    fn recursion(
        tree: &syn::UseTree,
        prefix: &[String],
        name: &str,
        anchor: &[String],
        hints: &mut Vec<Vec<String>>,
    ) {
        match tree {
            syn::UseTree::Path(path) => {
                let mut nested = prefix.to_vec();
                nested.push(path.ident.to_string());
                recursion(&path.tree, &nested, name, anchor, hints);
            }
            syn::UseTree::Group(group) => {
                for tree in &group.items {
                    recursion(tree, prefix, name, anchor, hints);
                }
            }
            syn::UseTree::Name(use_name) => {
                if use_name.ident == name {
                    if let Some(hint) =
                        rules::trait_impl_co_location::TraitImplCoLocation::defining_module_path(
                            prefix, anchor,
                        )
                    {
                        hints.push(hint);
                    }
                }
            }
            syn::UseTree::Rename(rename) => {
                if rename.ident == name || rename.rename == name {
                    if let Some(hint) =
                        rules::trait_impl_co_location::TraitImplCoLocation::defining_module_path(
                            prefix, anchor,
                        )
                    {
                        hints.push(hint);
                    }
                }
            }
            syn::UseTree::Glob(_) => {}
        }
    }
    let mut usages: HashMap<(String, PathBuf), HashSet<PathBuf>> = HashMap::new();
    let defined_names: HashSet<String> = defined.keys().cloned().collect();
    for (rel_path, file) in parsed_files {
        if rel_path.components().any(|c| c.as_os_str() == "testing") {
            continue;
        }
        let mut visitor = rules::constants_placement::ConstUseVisitor {
            defined: &defined_names,
            found: Vec::new(),
        };
        visitor.visit_file(file);
        for segments in visitor.found {
            let Some(name) = segments.last().cloned() else {
                continue;
            };
            let Some(candidates) = defined.get(&name) else {
                continue;
            };
            let Some(target) = (if let [single] = candidates.as_slice() {
                Some(single.clone())
            } else {
                match segments.split_last() {
                    Some((_, qualifier)) if !qualifier.is_empty() => {
                        let mut prefix = qualifier.to_vec();
                        while matches!(
                            prefix.first().map(String::as_str),
                            Some("crate" | "self" | "super")
                        ) {
                            prefix.remove(0);
                        }
                        let matched = candidates
                            .iter()
                            .filter(|candidate| {
                                rules::trait_impl_co_location::TraitImplCoLocation::impl_file_module_path(
                                    candidate,
                                )
                                .ends_with(&prefix)
                            })
                            .collect::<Vec<_>>();
                        match matched.as_slice() {
                            [single] => Some((*single).clone()),
                            _ => None,
                        }
                    }
                    _ => {
                        let anchor =
                            rules::trait_impl_co_location::TraitImplCoLocation::impl_file_module_path(
                                rel_path,
                            );
                        let mut hints = Vec::new();
                        for item in &file.items {
                            if let syn::Item::Use(item_use) = item {
                                recursion(&item_use.tree, &[], &name, &anchor, &mut hints);
                            }
                        }
                        let matched = candidates
                            .iter()
                            .filter(|candidate| {
                                hints.iter().any(|hint| {
                                    rules::trait_impl_co_location::TraitImplCoLocation::impl_file_module_path(
                                        candidate,
                                    )
                                    .ends_with(hint)
                                })
                            })
                            .collect::<Vec<_>>();
                        match matched.as_slice() {
                            [single] => Some((*single).clone()),
                            _ => None,
                        }
                    }
                }
            }) else {
                continue;
            };
            usages
                .entry((name, target))
                .or_default()
                .insert(rel_path.clone());
        }
    }
    usages
}

// no test_usage necessary
