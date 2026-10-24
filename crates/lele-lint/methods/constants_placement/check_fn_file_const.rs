use crate::rules;
use crate::Project;
use crate::RuleViolation;
use crate::Visibility;
use std::path::Path;

pub fn check_fn_file_const(name: &str, rel_path: &Path, project: &Project) -> Vec<RuleViolation> {
    let file_name = rel_path.file_name().and_then(|n| n.to_str()).unwrap_or("");
    if file_name == "constants.rs" || file_name == "mod.rs" || file_name == "lib.rs" {
        return Vec::new();
    }
    let Some(file) = project.parsed_files.get(rel_path) else {
        return Vec::new();
    };
    let has_pub_fn = file
        .items
        .iter()
        .any(|item| matches!(item, syn::Item::Fn(f) if Visibility(&f.vis).is_exposed()));
    if has_pub_fn {
        let sibling =
            rules::constants_placement::ConstantsPlacement::sibling_constants(rel_path, project);
        return vec![RuleViolation::from_file(
            project.src_dir.join(rel_path),
            rules::constants_placement::ConstantsPlacement::CODE,
            format!(
                "const `{name}` lives in fn-file `{}` — move it to `{}` (nearest constants.rs)",
                rel_path.display(),
                sibling.display(),
            ),
        )];
    }
    Vec::new()
}

// no test_usage necessary
