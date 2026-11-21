use crate::DirRole;
use crate::Origin;
use crate::Project;

pub fn has_tests_component(project: &Project, path: &std::path::Path) -> bool {
    matches!(
        project.dir_role(Origin::Src, &path.join("file.rs")),
        DirRole::Tests
    )
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::has_tests_component;
    use crate::Project;

    #[test]
    fn test_usage() {
        let project = Project::default();
        assert!(has_tests_component(&project, Path::new("src/tests/a.rs")));
        assert!(!has_tests_component(&project, Path::new("src/a.rs")));
    }
}
