use crate::Project;
use std::path::Path;

const EXEMPT_DIRS: [&str; 2] = ["bevy_systems", "tests"];

const EXEMPT_FILES: [&str; 5] = ["mod.rs", "lib.rs", "main.rs", "build.rs", "constants.rs"];

pub fn is_exempt_file(rel_path: &Path, project: &Project) -> bool {
    let file_name = rel_path.file_name().and_then(|n| n.to_str()).unwrap_or("");
    EXEMPT_FILES.contains(&file_name)
        || project.is_in_dunder_dir(rel_path)
        || rel_path.components().any(|component| {
            component
                .as_os_str()
                .to_str()
                .is_some_and(|dir| EXEMPT_DIRS.contains(&dir))
        })
}

// no test_usage necessary
