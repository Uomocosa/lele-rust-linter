use std::path::PathBuf;

use crate::Project;

pub(crate) fn crate_project() -> Project {
    Project {
        root: PathBuf::from("/crate"),
        src_dir: PathBuf::from("/crate/src"),
        ..Project::default()
    }
}

#[cfg(test)]
mod tests {
    use super::crate_project;
    use std::path::PathBuf;

    #[test]
    fn test_usage() {
        assert_eq!(crate_project().root, PathBuf::from("/crate"));
    }
}
