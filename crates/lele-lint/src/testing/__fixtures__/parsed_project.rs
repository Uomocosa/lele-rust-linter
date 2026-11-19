use std::path::PathBuf;

use crate::Project;

pub(crate) fn parsed_project(base: Project, files: &[(&str, &str)]) -> Project {
    let mut project = base;
    for (path, source) in files {
        let file: syn::File = syn::parse_str(source).unwrap();
        project.parsed_files.insert(PathBuf::from(path), file);
    }
    project
}

#[cfg(test)]
mod tests {
    use super::parsed_project;
    use crate::Project;

    #[test]
    fn test_usage() {
        let project = parsed_project(Project::default(), &[("a.rs", "pub struct A;")]);
        assert_eq!(project.parsed_files.len(), 1);
    }
}
