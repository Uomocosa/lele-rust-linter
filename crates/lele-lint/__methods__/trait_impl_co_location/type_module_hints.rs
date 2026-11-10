use std::path::Path;

use crate::rules;
use crate::Project;

pub fn type_module_hints(
    rel_path: &Path,
    hit: &rules::trait_impl_co_location::ImplHit,
    project: &Project,
) -> Vec<Vec<String>> {
    fn recursion(
        tree: &syn::UseTree,
        prefix: &[String],
        type_name: &str,
        hints: &mut Vec<Vec<String>>,
    ) {
        match tree {
            syn::UseTree::Path(path) => {
                let mut nested = prefix.to_vec();
                nested.push(path.ident.to_string());
                recursion(&path.tree, &nested, type_name, hints);
            }
            syn::UseTree::Group(group) => {
                for tree in &group.items {
                    recursion(tree, prefix, type_name, hints);
                }
            }
            syn::UseTree::Name(name) => {
                if let Some(hint) =
                    rules::trait_impl_co_location::TraitImplCoLocation::import_module_hint(
                        prefix,
                        &name.ident,
                        type_name,
                    )
                {
                    hints.push(hint);
                }
            }
            syn::UseTree::Rename(rename) => {
                if let Some(hint) =
                    rules::trait_impl_co_location::TraitImplCoLocation::import_module_hint(
                        prefix,
                        &rename.ident,
                        type_name,
                    )
                {
                    hints.push(hint);
                }
                if let Some(hint) =
                    rules::trait_impl_co_location::TraitImplCoLocation::import_module_hint(
                        prefix,
                        &rename.rename,
                        type_name,
                    )
                {
                    hints.push(hint);
                }
            }
            syn::UseTree::Glob(_) => {}
        }
    }

    let mut hints = Vec::new();
    if let Some(type_path) = &hit.type_path {
        if type_path.len() > 1 {
            let Some((_, qualifier)) = type_path.split_last() else {
                return hints;
            };
            if let Some(hint) =
                rules::trait_impl_co_location::TraitImplCoLocation::defining_module_path(qualifier)
            {
                hints.push(hint);
            }
        }
    }
    if let Some(file) = project.get_parsed(rel_path) {
        let Some(type_name) = hit.type_name.as_deref() else {
            return hints;
        };
        for item in &file.items {
            if let syn::Item::Use(item_use) = item {
                recursion(&item_use.tree, &[], type_name, &mut hints);
            }
        }
    }
    hints
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::path::PathBuf;

    use super::type_module_hints;
    use crate::rules;
    use crate::Project;

    fn hit(type_path: &[&str]) -> rules::trait_impl_co_location::ImplHit {
        rules::trait_impl_co_location::ImplHit {
            line: 3,
            trait_name: "Default".to_string(),
            type_name: Some(type_path.last().unwrap_or(&"").to_string()),
            type_path: (!type_path.is_empty()).then(|| {
                type_path
                    .iter()
                    .map(|segment| segment.to_string())
                    .collect()
            }),
            is_blanket: false,
        }
    }

    fn project_with(source: &str) -> Project {
        let mut project = Project::default();
        project.parsed_files.insert(
            PathBuf::from("dial/setup.rs"),
            syn::parse_str::<syn::File>(source).unwrap(),
        );
        project
    }

    #[test]
    fn test_usage() {
        let project = project_with("use crate::discovery::Config;\n\nimpl Default for Config {}\n");
        let mut hints = type_module_hints(Path::new("dial/setup.rs"), &hit(&["Config"]), &project);
        hints.sort();
        assert_eq!(hints, vec![vec!["discovery".to_string()]]);
    }

    #[test]
    fn test_usage_hints_from_the_named_self_path_target_a_module() {
        let project = project_with("use std::fmt;\n\nimpl Default for Config {}\n");
        let hints = type_module_hints(
            Path::new("dial/setup.rs"),
            &hit(&["crate", "direct_dial", "Config"]),
            &project,
        );
        assert_eq!(hints, vec![vec!["direct_dial".to_string()]]);
    }

    #[test]
    fn test_usage_grouped_and_renamed_imports_are_matched() {
        let project = project_with(
            "use crate::a::{Config, Item};\nuse crate::b::Config as Alias;\n\nimpl Default for Config {}\n",
        );
        let mut hints = type_module_hints(Path::new("dial/setup.rs"), &hit(&["Config"]), &project);
        hints.sort();
        assert_eq!(hints, vec![vec!["a".to_string()], vec!["b".to_string()]]);
    }

    #[test]
    fn test_usage_qualified_paths_and_imports_pile_up() {
        let project = project_with("use crate::discovery::Config;\n\nimpl Default for Config {}\n");
        let mut hints = type_module_hints(
            Path::new("dial/setup.rs"),
            &hit(&["crate", "discovery", "Config"]),
            &project,
        );
        hints.sort();
        assert_eq!(
            hints,
            vec![vec!["discovery".to_string()], vec!["discovery".to_string()]]
        );
    }

    #[test]
    fn test_usage_files_without_a_match_hint_nothing() {
        let project = project_with("use std::fmt;\n\nimpl Default for Dial {}\n");
        let hints = type_module_hints(Path::new("dial/setup.rs"), &hit(&["Config"]), &project);
        assert_eq!(hints, Vec::<Vec<String>>::new());
    }
}

// no test_usage necessary
