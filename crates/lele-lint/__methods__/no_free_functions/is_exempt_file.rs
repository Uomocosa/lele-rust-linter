use crate::CodeDunderRole;
use crate::Origin;
use crate::Project;
use std::path::Path;

const EXEMPT_FILES: [&str; 5] = ["mod.rs", "lib.rs", "main.rs", "build.rs", "constants.rs"];

pub fn is_exempt_file(rel_path: &Path, project: &Project) -> bool {
    let file_name = rel_path.file_name().and_then(|n| n.to_str()).unwrap_or("");
    if EXEMPT_FILES.contains(&file_name) {
        return true;
    }
    let facts = project.dir_facts(Origin::Src, rel_path);
    facts.code_dunder == Some(CodeDunderRole::BevySystems)
        || facts.is_in_tests
        || facts.testing_role.is_some()
        || facts.is_in_container
}

// no test_usage necessary
