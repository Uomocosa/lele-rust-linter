use crate::rules;
use crate::Project;
use crate::RuleViolation;
use std::collections::HashSet;
use std::path::{Path, PathBuf};

pub fn check_lift_to_ancestor(
    name: &str,
    rel_path: &Path,
    users: &HashSet<PathBuf>,
    project: &Project,
) -> Vec<RuleViolation> {
    let ancestor = rules::constants_placement::ConstantsPlacement::common_ancestor_dir(users);
    let def_dir = rules::constants_placement::ConstantsPlacement::parent_dir(rel_path);
    if !ancestor.is_empty() || def_dir.is_empty() {
        return Vec::new();
    }
    let target =
        rules::constants_placement::ConstantsPlacement::constants_target(project, &PathBuf::new());
    if target == *rel_path {
        return Vec::new();
    }
    vec![RuleViolation::from_file(
        project.src_dir.join(rel_path),
        rules::constants_placement::ConstantsPlacement::CODE,
        format!(
            "const `{name}` is used across top-level dirs but lives in `{}` — lift it to `src/{}`",
            rel_path.display(),
            target.display(),
        ),
    )]
}

// no test_usage necessary
