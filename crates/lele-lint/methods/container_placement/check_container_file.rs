use crate::rules;
use crate::Project;
use crate::RuleViolation;
use std::path::Path;

pub fn check_container_file(
    rel_path: &Path,
    file: &syn::File,
    project: &Project,
    file_path: &Path,
) -> Vec<RuleViolation> {
    let Some(stem) = rel_path.file_stem().and_then(|s| s.to_str()) else {
        return Vec::new();
    };
    if stem == "mod.rs" || project.is_dunder_file_path(rel_path) {
        return Vec::new();
    }
    let Some(role) = rules::container_placement::ContainerPlacement::role_of_stem(stem) else {
        return Vec::new();
    };

    let mut violations = Vec::new();
    for item in &file.items {
        let Some(name) = rules::container_placement::ContainerPlacement::exposed_name(item) else {
            continue;
        };
        if rules::container_placement::ContainerPlacement::has_behavior(file, &name) {
            violations.push(RuleViolation::from_file(file_path.to_path_buf(), rules::container_placement::ContainerPlacement::CODE, format!(
                    "`{name}` has impls; `{stem}.rs` holds behavior-free items only — move it to `{name}.rs`"
                )));
            continue;
        }
        let natural = rules::container_placement::ContainerPlacement::classify(item);
        if natural != Some(role) {
            let hint = natural.map_or("a non-container item".to_string(), |r| r.file().to_string());
            violations.push(RuleViolation::from_file(
                file_path.to_path_buf(),
                rules::container_placement::ContainerPlacement::CODE,
                format!("`{name}` is a {hint}; move it to `{hint}`"),
            ));
        }
    }
    violations
}

// no test_usage necessary
