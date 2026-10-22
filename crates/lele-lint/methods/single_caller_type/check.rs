use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use derive_more::{Deref, DerefMut};
use syn::visit::Visit;

use crate::checkers;
use crate::FnBody;
use crate::ImplBlock;
use crate::ItemMarker;
use crate::Project;
use crate::RuleViolation;

pub fn check(
    _self: &checkers::single_caller_type::SingleCallerType,
    project: &Project,
) -> Vec<RuleViolation> {
    let mut violations = Vec::new();
    let defined_types = collect_defined_types(&project.parsed_files);
    let defined_names: HashSet<String> = defined_types.iter().map(|(n, _)| n.clone()).collect();
    let embedded = collect_embedded_type_names(&project.parsed_files);
    let mut refs = collect_file_references(&project.parsed_files, &defined_names);
    refs.extend(collect_file_references(
        &project.methods_parsed_files,
        &defined_names,
    ));

    for (name, rel_path) in &defined_types {
        if project.is_exempt_source_path(rel_path) {
            continue;
        }
        if embedded.contains(name) {
            continue;
        }
        let Some(file) = project.parsed_files.get(rel_path) else {
            continue;
        };
        if has_atomic_delegates(file, name) {
            continue;
        }
        if has_real_inherent_method(file, name) || has_conversion_impl(file, name) {
            continue;
        }

        let callers: Vec<&PathBuf> = refs
            .iter()
            .filter(|(f, names)| *f != rel_path && names.contains(name))
            .map(|(f, _)| f)
            .collect();

        if let [caller] = callers.as_slice() {
            violations.push(RuleViolation::from_file(project.src_dir.join(rel_path), checkers::single_caller_type::SingleCallerType::CODE, format!(
                    "type `{name}` has exactly one caller in `{}` and no atomic-delegate methods — define it in the caller's file instead of its own file",
                    caller.display()
                )));
        }
    }

    violations
}

// needed helper: type definition collection across all files
fn collect_defined_types(parsed_files: &HashMap<PathBuf, syn::File>) -> Vec<(String, PathBuf)> {
    let mut types = Vec::new();
    for (rel_path, file) in parsed_files {
        for item in &file.items {
            let name = match item {
                syn::Item::Struct(s) => Some(s.ident.to_string()),
                syn::Item::Enum(e) => Some(e.ident.to_string()),
                _ => None,
            };
            if let Some(name) = name {
                types.push((name, rel_path.clone()));
            }
        }
    }
    types
}

// needed helper: embedded type name collection from field types
fn collect_embedded_type_names(parsed_files: &HashMap<PathBuf, syn::File>) -> HashSet<String> {
    let mut names = HashSet::new();
    for file in parsed_files.values() {
        for item in &file.items {
            match item {
                syn::Item::Struct(s) => {
                    for field in &s.fields {
                        collect_type_paths(&field.ty, &mut names);
                    }
                }
                syn::Item::Enum(e) => {
                    for variant in &e.variants {
                        for field in variant.fields.iter() {
                            collect_type_paths(&field.ty, &mut names);
                        }
                    }
                }
                _ => {}
            }
        }
    }
    names
}

// needed helper: type path visitor
fn collect_type_paths(ty: &syn::Type, names: &mut HashSet<String>) {
    #[derive(Deref, DerefMut)]
    struct Collect<'a>(&'a mut HashSet<String>);
    impl<'ast> syn::visit::Visit<'ast> for Collect<'_> {
        fn visit_path(&mut self, node: &'ast syn::Path) {
            if let Some(seg) = node.segments.last() {
                self.insert(seg.ident.to_string());
            }
            syn::visit::visit_path(self, node);
        }
    }
    Collect(names).visit_type(ty);
}

// needed helper: atomic delegate method presence check
fn has_atomic_delegates(file: &syn::File, type_name: &str) -> bool {
    file.items.iter().any(|item| {
        let syn::Item::Impl(impl_block) = item else {
            return false;
        };
        if ImplBlock(impl_block).self_type_last().as_deref() != Some(type_name) {
            return false;
        }
        impl_block
            .attrs
            .iter()
            .any(|attr| ItemMarker(attr).is_atomic_delegates())
            || has_fn_shell(impl_block)
            || impl_is_all_delegate(impl_block)
    })
}

// needed helper: per-function atomic shell presence check
fn has_fn_shell(impl_block: &syn::ItemImpl) -> bool {
    impl_block.items.iter().any(
        |item| matches!(item, syn::ImplItem::Fn(method) if method.attrs.iter().any(|attr| ItemMarker(attr).is_atomic_delegate())),
    )
}

// needed helper: inherent method with a real (non-delegate) body in the type's file
fn has_real_inherent_method(file: &syn::File, type_name: &str) -> bool {
    file.items.iter().any(|item| {
        let syn::Item::Impl(impl_block) = item else {
            return false;
        };
        if impl_block.trait_.is_some() {
            return false;
        }
        if ImplBlock(impl_block).self_type_last().as_deref() != Some(type_name) {
            return false;
        }
        impl_block.items.iter().any(|item| {
            matches!(
                item,
                syn::ImplItem::Fn(method)
                    if !method.block.stmts.is_empty() && !FnBody(&method.block).is_delegate_call()
            )
        })
    })
}

// needed helper: `conversion_is_from` moves constructors into `From`/`TryFrom`, which count as behaviour
fn has_conversion_impl(file: &syn::File, type_name: &str) -> bool {
    file.items.iter().any(|item| {
        let syn::Item::Impl(impl_block) = item else {
            return false;
        };
        let is_conversion = impl_block
            .trait_
            .as_ref()
            .and_then(|(_, path, _)| path.segments.last())
            .is_some_and(|segment| segment.ident == "From" || segment.ident == "TryFrom");
        is_conversion && ImplBlock(impl_block).self_type_last().as_deref() == Some(type_name)
    })
}

// needed helper: all-delegate impl block verification
fn impl_is_all_delegate(impl_block: &syn::ItemImpl) -> bool {
    let mut has_fn = false;
    for item in &impl_block.items {
        if let syn::ImplItem::Fn(method) = item {
            has_fn = true;
            let [stmt] = method.block.stmts.as_slice() else {
                return false;
            };
            let expr = match stmt {
                syn::Stmt::Expr(e, _) => e,
                _ => return false,
            };
            let call = match expr {
                syn::Expr::Call(c) => c,
                syn::Expr::Await(a) => match &*a.base {
                    syn::Expr::Call(c) => c,
                    _ => return false,
                },
                _ => return false,
            };
            if let syn::Expr::Path(path) = call.func.as_ref() {
                let len = path.path.segments.len();
                if !(2..=3).contains(&len) {
                    return false;
                }
                continue;
            }
            return false;
        }
    }
    has_fn
}

// needed helper: per-file type reference collector
fn collect_file_references(
    parsed_files: &HashMap<PathBuf, syn::File>,
    defined: &HashSet<String>,
) -> HashMap<PathBuf, HashSet<String>> {
    let mut per_file = HashMap::new();
    for (rel_path, file) in parsed_files {
        let mut found = HashSet::new();
        collect_refs_from_items(&file.items, defined, &mut found);
        per_file.insert(rel_path.clone(), found);
    }
    per_file
}

// needed helper: item-level reference collection with cfg(test) skip
fn collect_refs_from_items(
    items: &[syn::Item],
    defined: &HashSet<String>,
    found: &mut HashSet<String>,
) {
    struct RefCollector<'a> {
        defined: &'a HashSet<String>,
        found: &'a mut HashSet<String>,
    }
    impl<'ast> syn::visit::Visit<'ast> for RefCollector<'_> {
        fn visit_item_mod(&mut self, node: &'ast syn::ItemMod) {
            if node.attrs.iter().any(|attr| ItemMarker(attr).is_cfg_test()) {
                return;
            }
            syn::visit::visit_item_mod(self, node);
        }

        fn visit_item_use(&mut self, node: &'ast syn::ItemUse) {
            if matches!(node.vis, syn::Visibility::Public(_)) {
                return;
            }
            syn::visit::visit_item_use(self, node);
        }

        fn visit_path(&mut self, node: &'ast syn::Path) {
            if let Some(seg) = node.segments.last() {
                if self.defined.contains(&seg.ident.to_string()) {
                    self.found.insert(seg.ident.to_string());
                }
            }
            syn::visit::visit_path(self, node);
        }
    }

    let mut collector = RefCollector { defined, found };
    for item in items {
        collector.visit_item(item);
    }
}

#[cfg(test)]
mod tests {
    use super::{check, collect_defined_types, collect_embedded_type_names, has_atomic_delegates};
    use crate::checkers;
    use crate::Project;
    use std::collections::HashMap;

    #[test]
    fn test_usage() {
        let checker = checkers::single_caller_type::SingleCallerType;
        let project = Project::default();
        assert!(check(&checker, &project).is_empty());
    }

    #[test]
    fn test_usage_defined_types() {
        let mut map = HashMap::new();
        map.insert(
            std::path::PathBuf::from("player.rs"),
            syn::parse_str("pub struct Player { pub x: u32 }\n").unwrap(),
        );
        let types = collect_defined_types(&map);
        assert_eq!(types.len(), 1);
        assert_eq!(types[0].0, "Player");
    }

    #[test]
    fn test_usage_enum_defined() {
        let mut map = HashMap::new();
        map.insert(
            std::path::PathBuf::from("msg.rs"),
            syn::parse_str("pub enum Msg { Ping, Quit }\n").unwrap(),
        );
        let types = collect_defined_types(&map);
        assert_eq!(types.len(), 1);
        assert_eq!(types[0].0, "Msg");
    }

    #[test]
    fn test_usage_embedded() {
        let mut map = HashMap::new();
        map.insert(
            std::path::PathBuf::from("config.rs"),
            syn::parse_str("pub struct Config { pub sec: Option<LeleTomlLintSections> }\n")
                .unwrap(),
        );
        let embedded = collect_embedded_type_names(&map);
        assert!(embedded.contains("Option"));
        assert!(embedded.contains("LeleTomlLintSections"));
    }

    #[test]
    fn test_usage_has_atomic_delegates() {
        let file: syn::File = syn::parse_str(
            "#[rustfmt::skip] impl Foo { pub fn new() -> Self { config_new::new() } }",
        )
        .unwrap();
        assert!(has_atomic_delegates(&file, "Foo"));
    }

    #[test]
    fn test_usage_fn_shell_counts_as_atomic() {
        let file: syn::File =
            syn::parse_str("impl Foo { #[atomic_delegate(Foo)] pub fn run(&self) {} }").unwrap();
        assert!(has_atomic_delegates(&file, "Foo"));
    }

    #[test]
    fn test_usage_no_atomic_delegate() {
        let file: syn::File =
            syn::parse_str("impl Foo { pub fn new() -> Self { Self { x: 1 } } }").unwrap();
        assert!(!has_atomic_delegates(&file, "Foo"));
    }

    #[test]
    fn test_usage_real_inherent_method_is_exempt() {
        let file: syn::File =
            syn::parse_str("impl Foo { pub const fn new() -> Self { Self } }").unwrap();
        assert!(super::has_real_inherent_method(&file, "Foo"));

        let delegated: syn::File =
            syn::parse_str("impl Foo { pub fn new() -> Self { foo_new::new() } }").unwrap();
        assert!(!super::has_real_inherent_method(&delegated, "Foo"));
    }

    #[test]
    fn test_usage_conversion_impl_is_exempt() {
        let from: syn::File =
            syn::parse_str("impl From<u64> for Foo { fn from(x: u64) -> Self { Self(x) } }")
                .unwrap();
        assert!(super::has_conversion_impl(&from, "Foo"));
        let display: syn::File =
            syn::parse_str("impl Display for Foo { fn fmt(&self) {} }").unwrap();
        assert!(!super::has_conversion_impl(&display, "Foo"));
    }
}

// no test_usage necessary
