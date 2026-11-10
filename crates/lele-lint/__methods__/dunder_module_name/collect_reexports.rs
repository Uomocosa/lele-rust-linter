use syn::spanned::Spanned;

use crate::rules;

pub fn collect_reexports(
    visitor: &mut rules::dunder_module_name::DunderNameVisitor,
    item: &syn::ItemUse,
) {
    fn recursion(
        tree: &syn::UseTree,
        prefix: &mut Vec<String>,
        leaves: &mut Vec<(Vec<String>, String, bool)>,
    ) {
        match tree {
            syn::UseTree::Path(path) => {
                prefix.push(path.ident.to_string());
                recursion(&path.tree, prefix, leaves);
                prefix.pop();
            }
            syn::UseTree::Name(name) => {
                let mut segments = prefix.clone();
                segments.push(name.ident.to_string());
                leaves.push((segments, name.ident.to_string(), false));
            }
            syn::UseTree::Rename(rename) => {
                let mut segments = prefix.clone();
                segments.push(rename.ident.to_string());
                leaves.push((segments, rename.rename.to_string(), false));
            }
            syn::UseTree::Glob(_) => {
                leaves.push((prefix.clone(), "*".to_string(), true));
            }
            syn::UseTree::Group(group) => {
                for tree in &group.items {
                    recursion(tree, prefix, leaves);
                }
            }
        }
    }
    let mut leaves: Vec<(Vec<String>, String, bool)> = Vec::new();
    let mut prefix = Vec::new();
    recursion(&item.tree, &mut prefix, &mut leaves);
    let line = item.span().start().line;
    for (segments, leaf_item, is_glob) in leaves {
        let Some(mount) = visitor.code_dunder_mount(&segments) else {
            continue;
        };
        let raw = if is_glob {
            let mut without_leaf = segments.clone();
            without_leaf.pop();
            format!("{}::*", without_leaf.join("::"))
        } else {
            segments.join("::")
        };
        let suggestion = rules::dunder_module_name::DunderModuleName::reexport_suggestion(
            &visitor.domain_prefix,
            &segments,
            is_glob,
            &mount,
        );
        visitor
            .reexports
            .push(rules::dunder_module_name::ReexportHit {
                line,
                mount,
                item: leaf_item,
                raw,
                suggestion,
            });
    }
}

#[cfg(test)]
mod tests {
    use super::collect_reexports;
    use crate::rules;
    use crate::DunderPath;
    use crate::Project;
    use std::path::Path;
    use std::path::PathBuf;
    use syn::visit::Visit;

    fn hits(rel_path: &str, source: &str) -> Vec<String> {
        let mut project = Project {
            dunder_paths: DunderPath::defaults(),
            ..Project::default()
        };
        project
            .parsed_files
            .insert(PathBuf::from(rel_path), syn::parse_str(source).unwrap());
        let file = project.get_parsed(Path::new(rel_path)).unwrap().clone();
        let Some(syn::Item::Use(item_use)) = file.items.first() else {
            return Vec::new();
        };
        let mut hits = Vec::new();
        let mut reexports = Vec::new();
        let mut visitor = rules::dunder_module_name::DunderNameVisitor {
            dunders: &project.dunder_paths,
            hits: &mut hits,
            reexports: &mut reexports,
            domain_prefix: rules::dunder_module_name::DunderModuleName::mount_prefix(
                Path::new(rel_path),
                &project,
            ),
        };
        collect_reexports(&mut visitor, item_use);
        reexports.iter().map(|hit| hit.suggestion.clone()).collect()
    }

    #[test]
    fn test_usage() {
        let found = hits("greeting/mod.rs", "pub use traits::Greetable;\n");
        assert_eq!(found, vec!["greeting::traits::Greetable".to_string()]);
    }

    #[test]
    fn test_usage_visitor_skips_private_uses() {
        let mut project = Project {
            dunder_paths: DunderPath::defaults(),
            ..Project::default()
        };
        project.parsed_files.insert(
            PathBuf::from("guest.rs"),
            syn::parse_str("use crate::stock::traits::Clock;\n").unwrap(),
        );
        let file = project.get_parsed(Path::new("guest.rs")).unwrap().clone();
        let mut hits = Vec::new();
        let mut reexports = Vec::new();
        let mut visitor = rules::dunder_module_name::DunderNameVisitor {
            dunders: &project.dunder_paths,
            hits: &mut hits,
            reexports: &mut reexports,
            domain_prefix: rules::dunder_module_name::DunderModuleName::mount_prefix(
                Path::new("guest.rs"),
                &project,
            ),
        };
        visitor.visit_file(&file);
        assert_eq!(reexports.len(), 0);
    }
}
