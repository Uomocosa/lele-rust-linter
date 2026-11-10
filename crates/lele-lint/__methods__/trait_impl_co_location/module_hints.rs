use std::path::Path;

use crate::rules;
use crate::Project;

pub fn module_hints(
    rel_path: &Path,
    name: &str,
    item_path: Option<&[String]>,
    project: &Project,
) -> Vec<Vec<String>> {
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
                if let Some(hint) =
                    rules::trait_impl_co_location::TraitImplCoLocation::import_module_hint(
                        prefix,
                        &use_name.ident,
                        name,
                        anchor,
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
                        name,
                        anchor,
                    )
                {
                    hints.push(hint);
                }
                if let Some(hint) =
                    rules::trait_impl_co_location::TraitImplCoLocation::import_module_hint(
                        prefix,
                        &rename.rename,
                        name,
                        anchor,
                    )
                {
                    hints.push(hint);
                }
            }
            syn::UseTree::Glob(_) => {}
        }
    }

    let mut hints = Vec::new();
    let anchor =
        rules::trait_impl_co_location::TraitImplCoLocation::impl_file_module_path(rel_path);
    if let Some(item_path) = item_path {
        if item_path.len() > 1 {
            let Some((_, qualifier)) = item_path.split_last() else {
                return hints;
            };
            if let Some(hint) =
                rules::trait_impl_co_location::TraitImplCoLocation::defining_module_path(
                    qualifier, &anchor,
                )
            {
                hints.push(hint);
            }
        }
    }
    if let Some(file) = project.get_parsed(rel_path) {
        for item in &file.items {
            if let syn::Item::Use(item_use) = item {
                recursion(&item_use.tree, &[], name, &anchor, &mut hints);
            }
        }
    }
    hints
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::path::PathBuf;

    use super::module_hints;
    use crate::rules;
    use crate::Project;

    fn hit(type_path: &[&str]) -> rules::trait_impl_co_location::ImplHit {
        rules::trait_impl_co_location::ImplHit {
            line: 3,
            trait_name: "Default".to_string(),
            trait_path: Some(vec!["Default".to_string()]),
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

    fn path_strings(hints: &[Vec<String>]) -> Vec<Vec<String>> {
        let mut hints = hints.to_vec();
        hints.sort();
        hints
    }

    #[test]
    fn test_usage() {
        let project = project_with("use crate::discovery::Config;\n\nimpl Default for Config {}\n");
        let hit = hit(&["Config"]);
        let hints = module_hints(
            Path::new("dial/setup.rs"),
            hit.type_name.as_deref().unwrap_or_default(),
            hit.type_path.as_deref(),
            &project,
        );
        assert_eq!(path_strings(&hints), vec![vec!["discovery".to_string()]]);
    }

    #[test]
    fn test_usage_hints_from_the_named_item_path_target_a_module() {
        let project = project_with("use std::fmt;\n\nimpl Default for Config {}\n");
        let hints = module_hints(
            Path::new("dial/setup.rs"),
            "Config",
            Some(&[
                "crate".to_string(),
                "direct_dial".to_string(),
                "Config".to_string(),
            ]),
            &project,
        );
        assert_eq!(hints, vec![vec!["direct_dial".to_string()]]);
    }

    #[test]
    fn test_usage_grouped_and_renamed_imports_are_matched() {
        let project = project_with(
            "use crate::a::{Config, Item};\nuse crate::b::Config as Alias;\n\nimpl Default for Config {}\n",
        );
        let hints = module_hints(
            Path::new("dial/setup.rs"),
            "Config",
            Some(&["Config".to_string()]),
            &project,
        );
        assert_eq!(
            path_strings(&hints),
            vec![vec!["a".to_string()], vec!["b".to_string()]]
        );
    }

    #[test]
    fn test_usage_super_grouped_and_renamed_imports_hint_the_anchor_module() {
        let project =
            project_with("use super::x::{Foo as Bar, Item};\n\nimpl Default for Foo {}\n");
        let hints = module_hints(Path::new("dial/setup.rs"), "Foo", None, &project);
        assert_eq!(hints, vec![vec!["dial".to_string(), "x".to_string()]]);
    }

    #[test]
    fn test_usage_super_imports_hint_the_anchor_relative_module() {
        let project = project_with("use super::discovery::Config;\n\nimpl Default for Config {}\n");
        let hints = module_hints(Path::new("dial/setup.rs"), "Config", None, &project);
        assert_eq!(
            hints,
            vec![vec!["dial".to_string(), "discovery".to_string()]]
        );
    }

    #[test]
    fn test_usage_qualified_paths_and_imports_pile_up() {
        let project = project_with("use crate::discovery::Config;\n\nimpl Default for Config {}\n");
        let hints = module_hints(
            Path::new("dial/setup.rs"),
            "Config",
            Some(&[
                "crate".to_string(),
                "discovery".to_string(),
                "Config".to_string(),
            ]),
            &project,
        );
        assert_eq!(
            path_strings(&hints),
            vec![vec!["discovery".to_string()], vec!["discovery".to_string()]]
        );
    }

    #[test]
    fn test_usage_files_without_a_match_hint_nothing() {
        let project = project_with("use std::fmt;\n\nimpl Default for Dial {}\n");
        let hints = module_hints(
            Path::new("dial/setup.rs"),
            "Config",
            Some(&["Config".to_string()]),
            &project,
        );
        assert_eq!(hints, Vec::<Vec<String>>::new());
    }

    #[test]
    fn test_usage_hints_work_for_traits_through_the_trait_path() {
        let project = project_with("use crate::a::Foo;\n\nimpl crate::a::Foo for T {}\n");
        let hints = module_hints(
            Path::new("dial/setup.rs"),
            "Foo",
            Some(&["crate".to_string(), "a".to_string(), "Foo".to_string()]),
            &project,
        );
        assert_eq!(
            path_strings(&hints),
            vec![vec!["a".to_string()], vec!["a".to_string()]]
        );
    }
}
