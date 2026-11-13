use crate::common;
use crate::Project;
use std::collections::HashSet;

pub fn known_type_stems(project: &Project) -> HashSet<String> {
    project
        .parsed_files
        .iter()
        .filter_map(|(rel_path, file)| {
            let stem = rel_path.file_stem()?.to_str()?;
            common::SourceFile(file).primary_type_name(stem)?;
            Some(stem.to_string())
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::rules;
    use crate::Project;
    use std::path::PathBuf;

    #[test]
    fn test_usage() {
        let mut project = Project::default();
        project.parsed_files.insert(
            PathBuf::from("rules/atomic_file.rs"),
            syn::parse_str("pub struct AtomicFile;").unwrap(),
        );
        project.parsed_files.insert(
            PathBuf::from("rules/helpers.rs"),
            syn::parse_str("fn private() {}").unwrap(),
        );
        let stems = rules::atomic_file::AtomicFile::known_type_stems(&project);
        assert!(stems.contains("atomic_file"));
        assert!(!stems.contains("helpers"));
    }
}
