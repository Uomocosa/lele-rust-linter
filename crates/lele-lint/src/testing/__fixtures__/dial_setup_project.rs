use crate::testing;
use crate::Project;

pub(crate) fn dial_setup_project(source: &str) -> Project {
    testing::parsed_project(Project::default(), &[("dial/setup.rs", source)])
}

#[cfg(test)]
mod tests {
    use super::dial_setup_project;

    #[test]
    fn test_usage() {
        assert_eq!(
            dial_setup_project("impl Default for Config {}")
                .parsed_files
                .len(),
            1
        );
    }
}
