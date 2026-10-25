use crate::rules;
use crate::RuleViolation;
use std::path::Path;

pub fn check_free_file(
    rel_path: &Path,
    file: &syn::File,
    folder: &str,
    file_path: &Path,
) -> Vec<RuleViolation> {
    let Some(stem) = rel_path.file_stem().and_then(|s| s.to_str()) else {
        return Vec::new();
    };
    if stem == "mod.rs" || stem == "lib.rs" {
        return Vec::new();
    }
    let target = rules::container_placement::ContainerPlacement::container_dir(rel_path, folder);

    let mut violations = Vec::new();
    for item in &file.items {
        let Some(name) = rules::container_placement::ContainerPlacement::exposed_name(item) else {
            continue;
        };
        if rules::container_placement::ContainerPlacement::has_behavior(file, &name) {
            continue;
        }
        let Some(role) = rules::container_placement::ContainerPlacement::classify(item) else {
            continue;
        };
        if role == rules::container_placement::Role::Const {
            continue;
        }
        violations.push(RuleViolation::from_file(
            file_path.to_path_buf(),
            rules::container_placement::ContainerPlacement::CODE,
            format!(
                "`{name}` is a behavior-free type outside `__basic__/` — move it to `{}`",
                target.join(role.file()).display(),
            ),
        ));
    }
    violations
}

// no test_usage necessary
