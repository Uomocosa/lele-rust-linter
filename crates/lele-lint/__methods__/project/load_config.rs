use crate::config;
use crate::Config;
use crate::Error;
use crate::Project;

pub fn load_config(project: &Project) -> Result<Config, Error> {
    let config_path = project.root.join(config::CONFIG_FILENAME);
    match Config::try_from(config_path.as_path()) {
        Err(Error::Io(error)) if error.kind() == std::io::ErrorKind::NotFound => {
            Ok(Config::default())
        }
        loaded => loaded,
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::Project;

    #[test]
    fn test_usage() {
        let missing = Project {
            root: PathBuf::from("/nonexistent-lele-lint-probe"),
            ..Project::default()
        };
        assert!(missing.load_config().unwrap().as_ref().is_none());
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("lele.toml"), "[lele.lint]\n").unwrap();
        let present = Project {
            root: dir.path().to_path_buf(),
            ..Project::default()
        };
        assert!(present.load_config().unwrap().as_ref().is_some());
    }
}
