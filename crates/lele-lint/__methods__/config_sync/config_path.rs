use std::path::Path;
use std::path::PathBuf;

use crate::config;
use crate::Project;

pub fn config_path(explicit: Option<&Path>, project: &Project) -> PathBuf {
    if let Some(path) = explicit {
        return path.to_path_buf();
    }
    let cwd = std::env::current_dir()
        .map(|dir| dir.join(config::CONFIG_FILENAME))
        .ok();
    match cwd {
        Some(path) if path.exists() => path,
        _ => project.root.join(config::CONFIG_FILENAME),
    }
}

#[cfg(test)]
mod tests {
    use super::config_path;
    use crate::Project;

    #[test]
    fn test_usage() {
        let project = Project {
            root: std::path::PathBuf::from("/nonexistent-lele-sync-probe"),
            ..Project::default()
        };
        let explicit = std::path::PathBuf::from("custom.toml");
        assert_eq!(config_path(Some(explicit.as_path()), &project), explicit);
        let fallback = config_path(None, &project);
        assert!(fallback.ends_with("lele.toml"));
    }
}
