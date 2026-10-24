use std::path::Path;
use std::path::PathBuf;

use crate::Config;
use crate::Error;
use crate::Project;

pub fn resolve(explicit: Option<&Path>, project: &Project) -> Result<(Config, PathBuf), Error> {
    let project_root = project.root.as_path();
    if let Some(path) = explicit {
        return Config::load_at(path, project_root);
    }
    let cwd = std::env::current_dir()
        .ok()
        .map(|dir| dir.join("lele.toml"))
        .filter(|path| path.exists());
    match cwd {
        Some(path) => Config::load_at(&path, project_root),
        None => Ok((project.load_config()?, project_root.to_path_buf())),
    }
}

// no test_usage necessary
