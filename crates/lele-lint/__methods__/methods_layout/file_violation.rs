use crate::rules;
use crate::Project;
use crate::RuleViolation;
use std::path::Path;

pub fn file_violation(project: &Project, rel: &Path, message: String) -> RuleViolation {
    let file = project
        .methods_dir
        .as_ref()
        .map_or_else(|| rel.to_path_buf(), |dir| dir.join(rel));
    RuleViolation::from_file(file, rules::methods_layout::MethodsLayout::CODE, message)
}

// no test_usage necessary
