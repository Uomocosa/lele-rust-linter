use crate::rules;
use crate::Project;
use crate::RuleViolation;
use std::collections::HashSet;
use std::path::{Path, PathBuf};

pub fn check_lower_to_subdir(
    name: &str,
    rel_path: &Path,
    users: &HashSet<PathBuf>,
    project: &Project,
) -> Vec<RuleViolation> {
    if rel_path.components().count() != 1 {
        return Vec::new();
    }
    let ancestor = rules::constants_placement::ConstantsPlacement::common_ancestor_dir(users);
    if ancestor.as_os_str().is_empty() {
        return Vec::new();
    }
    let target =
        rules::constants_placement::ConstantsPlacement::constants_target(project, &ancestor);
    vec![RuleViolation::from_file(
        project.src_dir.join(rel_path),
        rules::constants_placement::ConstantsPlacement::CODE,
        format!(
            "const `{name}` lives at root but is only used under `{}` — lower it to `{}`",
            ancestor.display(),
            target.display(),
        ),
    )]
}

// no test_usage necessary
