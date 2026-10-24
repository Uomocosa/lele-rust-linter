use crate::Project;
use std::path::PathBuf;

pub fn find_workspace_root(project: &Project) -> Option<PathBuf> {
    let mut cur = project.root.parent().map(|p| p.to_path_buf());
    while let Some(dir) = cur {
        if dir.join("Cargo.toml").exists() {
            if let Ok(content) = std::fs::read_to_string(dir.join("Cargo.toml")) {
                if let Ok(val) = content.parse::<toml::Value>() {
                    if val.get("workspace").is_some() {
                        return Some(dir);
                    }
                }
            }
        }
        let parent = dir.parent().map(|p| p.to_path_buf());
        if parent.is_none() || parent == Some(dir.clone()) {
            break;
        }
        cur = parent;
    }
    None
}

// no test_usage necessary
