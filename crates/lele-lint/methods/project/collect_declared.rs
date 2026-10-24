use lele_snake_case::ToLeleSnakeCase;
use std::collections::BTreeMap;

use crate::common;
use crate::ImplBlock;
use crate::ItemMarker;
use crate::Project;

pub fn collect_declared(project: &Project) -> BTreeMap<String, common::DeclaredType> {
    let mut map: BTreeMap<String, common::DeclaredType> = BTreeMap::new();
    let cfg_map = Project::module_cfgs(&project.module_info);
    for (rel_path, file) in &project.parsed_files {
        let Some(stem) = rel_path.file_stem().and_then(|s| s.to_str()) else {
            continue;
        };
        let Some(primary) = common::SourceFile(file).primary_type_name(stem) else {
            continue;
        };
        let type_snake = primary.to_snake_case();
        for item in &file.items {
            let syn::Item::Impl(impl_block) = item else {
                continue;
            };
            if ImplBlock(impl_block).self_type_last().as_deref() != Some(primary.as_str()) {
                continue;
            }
            let block_annotated = impl_block
                .attrs
                .iter()
                .any(|attr| ItemMarker(attr).is_atomic_delegates());
            for impl_item in &impl_block.items {
                if let syn::ImplItem::Fn(method) = impl_item {
                    if block_annotated
                        || method
                            .attrs
                            .iter()
                            .any(|attr| ItemMarker(attr).is_atomic_delegate())
                    {
                        let entry = map.entry(type_snake.clone()).or_default();
                        if entry.cfgs.is_empty() {
                            entry.cfgs = Project::file_cfgs(&cfg_map, rel_path);
                        }
                        entry
                            .methods
                            .insert(ImplBlock(impl_block).delegate_body_name(&method.sig.ident));
                    }
                }
            }
        }
    }
    map
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::collect_declared;
    use crate::Project;

    #[test]
    fn test_usage() {
        let mut project = Project::default();
        let file: syn::File = syn::parse_str(
            "pub struct ClickCounter(pub i32);\n#[atomic_delegates]\nimpl ClickCounter { pub fn add(&mut self) {} }",
        )
        .unwrap();
        project
            .parsed_files
            .insert(PathBuf::from("clicker/click_counter.rs"), file);
        let declared = collect_declared(&project);
        assert!(declared
            .get("click_counter")
            .is_some_and(|d| d.methods.contains("add")));
    }

    #[test]
    fn test_usage_collects_fn_shell() {
        let mut project = Project::default();
        let file: syn::File = syn::parse_str(
            "pub struct Foo;\nimpl Foo { #[atomic_delegate(Foo)] pub fn run(&mut self) {} }",
        )
        .unwrap();
        project.parsed_files.insert(PathBuf::from("foo.rs"), file);
        let declared = collect_declared(&project);
        assert!(declared
            .get("foo")
            .is_some_and(|d| d.methods.contains("run")));
    }

    #[test]
    fn test_usage_collects_annotated_trait_shell() {
        let mut project = Project::default();
        let file: syn::File = syn::parse_str(
            "pub struct Foo;\n#[atomic_delegates]\nimpl Rule for Foo { fn check(&self) {} }",
        )
        .unwrap();
        project.parsed_files.insert(PathBuf::from("foo.rs"), file);
        let declared = collect_declared(&project);
        assert!(declared
            .get("foo")
            .is_some_and(|d| d.methods.contains("check")));
    }

    #[test]
    fn test_usage_names_generic_trait_shell_after_type_args() {
        let mut project = Project::default();
        let file: syn::File = syn::parse_str(
            "pub struct Temperature;\n#[atomic_delegates]\nimpl From<Celsius> for Temperature { fn from(c: Celsius) -> Self {} }\n#[atomic_delegates]\nimpl From<Kelvin> for Temperature { fn from(k: Kelvin) -> Self {} }",
        )
        .unwrap();
        project
            .parsed_files
            .insert(PathBuf::from("temperature.rs"), file);
        let declared = collect_declared(&project);
        let methods = &declared["temperature"].methods;
        assert!(methods.contains("from_celsius") && methods.contains("from_kelvin"));
    }
}
