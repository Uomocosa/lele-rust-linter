use std::path::PathBuf;

use crate::CollectionWrapperEntry;
use crate::Project;

pub(crate) fn collection_wrapper_project(
    files: &[(&str, &str)],
    entries: Vec<CollectionWrapperEntry>,
) -> (tempfile::TempDir, Project) {
    let dir = tempfile::tempdir().unwrap();
    let mut project = Project {
        root: dir.path().to_path_buf(),
        src_dir: dir.path().join("src"),
        collection_wrappers: entries,
        ..Project::default()
    };
    for (rel, source) in files {
        let path = dir.path().join("src").join(rel);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(&path, source).unwrap();
        project
            .parsed_files
            .insert(PathBuf::from(rel), syn::parse_str(source).unwrap());
    }
    (dir, project)
}

#[cfg(test)]
mod tests {
    use super::collection_wrapper_project;

    #[test]
    fn test_usage() {
        let (_dir, project) =
            collection_wrapper_project(&[("cards/deck.rs", "pub struct Deck(Vec<Card>);")], vec![]);
        assert_eq!(project.parsed_files.len(), 1);
    }
}
