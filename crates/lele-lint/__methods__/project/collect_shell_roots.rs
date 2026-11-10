use lele_snake_case::traits::ToLeleSnakeCase;
use std::collections::BTreeMap;
use std::collections::BTreeSet;

use crate::common;
use crate::ImplBlock;
use crate::ItemMarker;
use crate::Project;

pub fn collect_shell_roots(project: &Project) -> BTreeMap<(String, String), BTreeSet<String>> {
    let mut map: BTreeMap<(String, String), BTreeSet<String>> = BTreeMap::new();
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
                        let body = ImplBlock(impl_block).delegate_body_name(&method.sig.ident);
                        map.entry((type_snake.clone(), body))
                            .or_default()
                            .insert(project.root_name_for_rel(rel_path));
                    }
                }
            }
        }
    }
    map
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;
    use std::path::PathBuf;

    use super::collect_shell_roots;
    use crate::Project;

    #[test]
    fn test_usage() {
        let dir = tempfile::tempdir().unwrap();
        let mut project = Project {
            root: dir.path().to_path_buf(),
            src_dir: dir.path().join("src"),
            extra_src_dirs: vec![dir.path().join("ui")],
            layout_roots: vec![PathBuf::from("src"), PathBuf::from("ui")],
            ..Project::default()
        };
        project.parsed_files.insert(
            PathBuf::from("widget.rs"),
            syn::parse_str(
                "pub struct Widget;\n#[atomic_delegates]\nimpl Widget { pub fn spin(&self) {} }",
            )
            .unwrap(),
        );
        project.parsed_files.insert(
            PathBuf::from("ui/gadget.rs"),
            syn::parse_str(
                "pub struct Gadget;\n#[atomic_delegates]\nimpl Gadget { pub fn fetch(&self) {} }",
            )
            .unwrap(),
        );
        let roots = collect_shell_roots(&project);
        assert_eq!(
            roots.get(&("widget".to_string(), "spin".to_string())),
            Some(&BTreeSet::from(["src".to_string()]))
        );
        assert_eq!(
            roots.get(&("gadget".to_string(), "fetch".to_string())),
            Some(&BTreeSet::from(["ui".to_string()]))
        );
    }
}
