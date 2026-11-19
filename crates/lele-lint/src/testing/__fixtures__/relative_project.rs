use std::path::PathBuf;

use crate::Project;

pub(crate) fn relative_project() -> Project {
    Project {
        root: PathBuf::from("."),
        src_dir: PathBuf::from("src"),
        ..Project::default()
    }
}

#[cfg(test)]
mod tests {
    use super::relative_project;
    use std::path::PathBuf;

    #[test]
    fn test_usage() {
        assert_eq!(relative_project().src_dir, PathBuf::from("src"));
    }
}
