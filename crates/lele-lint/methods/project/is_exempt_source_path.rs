use std::path::Path;

use crate::Project;

pub fn is_exempt_source_path(project: &Project, rel_path: &Path) -> bool {
    let file_name = rel_path.file_name().and_then(|n| n.to_str()).unwrap_or("");
    let is_exempt_file = matches!(file_name, "mod.rs" | "lib.rs" | "constants.rs");
    let in_tests = rel_path
        .components()
        .any(|c| c.as_os_str().to_str() == Some("tests"));
    is_exempt_file || project.is_in_dunder_dir(rel_path) || in_tests
}

#[cfg(test)]
mod tests {
    use super::is_exempt_source_path;
    use crate::Project;
    use std::path::Path;

    #[test]
    fn test_usage() {
        let project = Project::default();
        assert!(is_exempt_source_path(&project, Path::new("a/mod.rs")));
        assert!(is_exempt_source_path(&project, Path::new("a/constants.rs")));
        assert!(!is_exempt_source_path(&project, Path::new("a/foo.rs")));
    }
}
