use crate::DirRole;
use crate::Origin;
use crate::Project;
use std::path::Path;

const EXEMPT_FILES: [&str; 5] = ["mod.rs", "lib.rs", "main.rs", "build.rs", "constants.rs"];

pub fn is_exempt_file(rel_path: &Path, project: &Project) -> bool {
    let file_name = rel_path.file_name().and_then(|n| n.to_str()).unwrap_or("");
    if EXEMPT_FILES.contains(&file_name) {
        return true;
    }
    matches!(
        project.dir_role(Origin::Src, rel_path),
        DirRole::BevySystems | DirRole::Tests | DirRole::Testing(_) | DirRole::Container
    )
}

// no test_usage necessary
