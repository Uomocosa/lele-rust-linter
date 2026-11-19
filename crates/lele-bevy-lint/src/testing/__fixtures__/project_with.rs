use std::path::PathBuf;

use lele_lint::Project;

pub(crate) fn project_with(base: Project, sources: &[(&str, &str)]) -> Project {
    let mut project = base;
    for (path, source) in sources {
        let file: syn::File = syn::parse_str(source).unwrap();
        project.parsed_files.insert(PathBuf::from(path), file);
    }
    project
}

#[cfg(test)]
mod tests {
    use super::project_with;
    use lele_lint::Project;

    #[test]
    fn test_usage() {
        let project = project_with(Project::default(), &[("a.rs", "pub struct A;")]);
        assert_eq!(project.parsed_files.len(), 1);
    }
}
