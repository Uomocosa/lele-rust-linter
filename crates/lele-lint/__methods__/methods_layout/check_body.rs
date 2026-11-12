use std::path::Path;

use crate::rules;
use crate::FnBody;
use crate::Origin;
use crate::ParsedSource;
use crate::Project;
use crate::RawCode;
use crate::RuleViolation;

pub fn check_body(
    project: &Project,
    type_snake: &str,
    method: &str,
    rel: &Path,
    file: &syn::File,
) -> Vec<RuleViolation> {
    let mut violations = Vec::new();
    if !rules::methods_layout::MethodsLayout::defines_pub_fn(file, method) {
        violations.push(rules::methods_layout::MethodsLayout::file_violation(
            project,
            rel,
            format!(
                "`{}/{}` must define `pub fn {method}`",
                project.methods_root_name(),
                rel.display()
            ),
        ));
    } else if rules::methods_layout::MethodsLayout::pub_fn_block(file, method)
        .is_some_and(|block| FnBody(block).is_one_liner())
    {
        violations.push(rules::methods_layout::MethodsLayout::file_violation(
            project,
            rel,
            format!(
                "`{}/{}` body is a one-liner — inline `pub fn {method}` in `{type_snake}.rs` under `#[rustfmt::skip]`, then delete this file; never pad the body or change its signature to keep the file",
                project.methods_root_name(),
                rel.display()
            ),
        ));
    }
    if !(ParsedSource {
        origin: Origin::Methods,
        relative_path: rel,
        file,
    })
    .has_test_usage()
        && !rules::methods_layout::MethodsLayout::read_methods_file(project, rel)
            .is_some_and(|text| RawCode(&text).ends_with_test_usage_opt_out())
    {
        violations.push(rules::methods_layout::MethodsLayout::file_violation(
            project,
            rel,
            format!(
                "`{}/{}` must carry its own `#[cfg(test)]` `test_usage`",
                project.methods_root_name(),
                rel.display()
            ),
        ));
    }
    violations
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::path::PathBuf;

    use super::check_body;
    use crate::Project;

    #[test]
    fn test_usage() {
        let project = Project {
            methods_dir: Some(PathBuf::from("methods")),
            ..Project::default()
        };
        let file: syn::File = syn::parse_str(
            "pub fn spin(w: &mut Widget) {\n    let n = 1;\n    let _ = n;\n}\n#[cfg(test)] mod tests { #[test] fn test_usage() {} }\n",
        )
        .unwrap();
        assert!(check_body(
            &project,
            "widget",
            "spin",
            Path::new("widget/spin.rs"),
            &file
        )
        .is_empty());
        let missing: syn::File = syn::parse_str("pub fn other() {}\n").unwrap();
        let violations = check_body(
            &project,
            "widget",
            "spin",
            Path::new("widget/spin.rs"),
            &missing,
        );
        assert!(violations
            .iter()
            .any(|d| d.message.contains("must define `pub fn spin`")));
        let one_line: syn::File =
            syn::parse_str("pub fn spin(w: &mut Widget) { w.spin_once() }\n").unwrap();
        assert!(check_body(
            &project,
            "widget",
            "spin",
            Path::new("widget/spin.rs"),
            &one_line
        )
        .iter()
        .any(|d| d.message.contains("body is a one-liner")));
        let multi_line: syn::File = syn::parse_str(
            "pub fn spin(w: &mut Widget) {\n    match w.count {\n        0 => {}\n        _ => {}\n    }\n}\n#[cfg(test)] mod tests { #[test] fn test_usage() {} }\n",
        )
        .unwrap();
        assert!(!check_body(
            &project,
            "widget",
            "spin",
            Path::new("widget/spin.rs"),
            &multi_line
        )
        .iter()
        .any(|d| d.message.contains("body is a one-liner")));
    }
}
