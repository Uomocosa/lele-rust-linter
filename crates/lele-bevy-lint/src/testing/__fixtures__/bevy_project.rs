use lele_lint::DunderPath;
use lele_lint::Project;

use crate::testing;

pub(crate) fn bevy_project(sources: &[(&str, &str)]) -> Project {
    let base = Project {
        dunder_paths: DunderPath::defaults(),
        ..Project::default()
    };
    testing::project_with(base, sources)
}

#[cfg(test)]
mod tests {
    use super::bevy_project;
    use lele_lint::DunderPath;

    #[test]
    fn test_usage() {
        let project = bevy_project(&[("a.rs", "pub struct A;")]);
        assert_eq!(project.parsed_files.len(), 1);
        assert_eq!(project.dunder_paths.len(), DunderPath::defaults().len());
    }
}
