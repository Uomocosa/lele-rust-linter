use std::path::Path;
use std::path::PathBuf;

use crate::EntryPoint;
use crate::Project;

pub(crate) fn workspace_project(workspace: &Path, entry_file: &str) -> Project {
    let crate_src = workspace.join("crates/a/src");
    std::fs::create_dir_all(&crate_src).unwrap();
    std::fs::write(crate_src.join("run.rs"), "pub fn run() {}").unwrap();
    let mut project = Project {
        root: workspace.join("crates/a"),
        src_dir: crate_src,
        config_dir: workspace.to_path_buf(),
        ..Project::default()
    };
    project.parsed_files.insert(
        PathBuf::from("run.rs"),
        syn::parse_str("pub fn run() {}").unwrap(),
    );
    project.entry_points.push(EntryPoint {
        file: PathBuf::from(entry_file),
        function: "run".to_string(),
        reason: "task".to_string(),
    });
    project
}

#[cfg(test)]
mod tests {
    use super::workspace_project;

    #[test]
    fn test_usage() {
        let dir = tempfile::tempdir().unwrap();
        let project = workspace_project(dir.path(), "crates/a/src/run.rs");
        assert_eq!(project.entry_points.len(), 1);
    }
}
