use std::path::Path;
use std::path::PathBuf;

use crate::methods;
use crate::Project;
use crate::RuleViolation;

pub(crate) fn methods_body_violations(src: &str) -> Vec<RuleViolation> {
    let dir = tempfile::tempdir().unwrap();
    std::fs::create_dir_all(dir.path().join("methods/widget")).unwrap();
    std::fs::write(dir.path().join("methods/widget/spin.rs"), src).unwrap();
    let file: syn::File = syn::parse_str(src).unwrap();
    let mut project = Project {
        root: dir.path().to_path_buf(),
        methods_dir: Some(dir.path().join("methods")),
        ..Project::default()
    };
    project
        .methods_parsed_files
        .insert(PathBuf::from("widget/spin.rs"), file.clone());
    project.fill_rustfmt_answers().unwrap();
    methods::methods_layout::check_body(
        &project,
        "widget",
        "spin",
        Path::new("widget/spin.rs"),
        &file,
    )
}

#[cfg(test)]
mod tests {
    use super::methods_body_violations;

    #[test]
    fn test_usage() {
        assert!(methods_body_violations(
            "pub fn spin(w: &mut Widget) {\n    let n = 1;\n    let _ = n;\n}\n#[cfg(test)] mod tests { #[test] fn test_usage() {} }\n"
        )
        .is_empty());
    }
}
