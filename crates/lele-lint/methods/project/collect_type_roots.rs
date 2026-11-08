use lele_snake_case::ToLeleSnakeCase;
use std::collections::BTreeMap;
use std::path::PathBuf;

use crate::common;
use crate::Project;

pub fn collect_type_roots(project: &Project) -> BTreeMap<String, BTreeMap<String, PathBuf>> {
    let mut map: BTreeMap<String, BTreeMap<String, PathBuf>> = BTreeMap::new();
    for (rel_path, file) in &project.parsed_files {
        let Some(stem) = rel_path.file_stem().and_then(|s| s.to_str()) else {
            continue;
        };
        let Some(primary) = common::SourceFile(file).primary_type_name(stem) else {
            continue;
        };
        map.entry(primary.to_snake_case())
            .or_default()
            .insert(project.root_name_for_rel(rel_path), rel_path.clone());
    }
    map
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::path::PathBuf;

    use super::collect_type_roots;
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
            syn::parse_str("pub struct Widget;").unwrap(),
        );
        project.parsed_files.insert(
            PathBuf::from("ui/widget.rs"),
            syn::parse_str("pub struct Widget;").unwrap(),
        );
        let roots = collect_type_roots(&project);
        let widget = roots.get("widget").expect("widget roots");
        assert_eq!(widget.len(), 2);
        assert_eq!(widget.get("src"), Some(&PathBuf::from("widget.rs")));
        assert_eq!(widget.get("ui"), Some(&PathBuf::from("ui/widget.rs")));
    }

    #[test]
    fn test_usage_single_root_has_one_entry() {
        let mut project = Project::default();
        project.parsed_files.insert(
            PathBuf::from("widget.rs"),
            syn::parse_str("pub struct Widget;").unwrap(),
        );
        let roots = collect_type_roots(&project);
        assert_eq!(roots.get("widget").map(BTreeMap::len), Some(1));
    }
}
