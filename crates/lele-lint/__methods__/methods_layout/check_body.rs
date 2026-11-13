use std::path::Path;

use crate::rules;
use crate::FnBody;
use crate::Origin;
use crate::ParsedSource;
use crate::PlacementKind;
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
    } else if let Some(block) = rules::methods_layout::MethodsLayout::pub_fn_block(file, method) {
        let rustfmt_one_line = block
            .stmts
            .first()
            .is_some_and(|stmt| project.rustfmt_one_liner(rel, stmt));
        match FnBody(block).placement_kind(rustfmt_one_line) {
            PlacementKind::OneLiner => violations.push(rules::methods_layout::MethodsLayout::file_violation(
                project,
                rel,
                format!(
                    "`{}/{}` body is a one-liner — inline `pub fn {method}` in `{type_snake}.rs` under `#[rustfmt::skip]`, then delete this file; never pad the body or change its signature to keep the file",
                    project.methods_root_name(),
                    rel.display()
                ),
            )),
            PlacementKind::SimpleBlock => violations.push(rules::methods_layout::MethodsLayout::file_violation(
                project,
                rel,
                format!(
                    "`{}/{}` body is a single simple `match`/`if`/loop — inline `pub fn {method}` in `{type_snake}.rs` in a plain `impl` block (rustfmt formats it), then delete this file; never pad the body or change its signature to keep the file",
                    project.methods_root_name(),
                    rel.display()
                ),
            )),
            PlacementKind::FewStatements | PlacementKind::Delegate => {}
        }
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
    use crate::RuleViolation;

    fn run_body(src: &str) -> Vec<RuleViolation> {
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
        check_body(
            &project,
            "widget",
            "spin",
            Path::new("widget/spin.rs"),
            &file,
        )
    }

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
    }

    #[test]
    fn test_usage_one_liner_body_is_reported() {
        let violations = run_body(
            "pub fn spin(w: &mut Widget) { w.spin_once() }\n#[cfg(test)] mod tests { #[test] fn test_usage() {} }\n",
        );
        assert!(violations
            .iter()
            .any(|d| d.message.contains("body is a one-liner")));
    }

    #[test]
    fn test_usage_simple_match_body_is_reported() {
        let violations = run_body(
            "pub fn spin(w: &mut Widget) {\n    match w.count {\n        0 => {}\n        _ => {}\n    }\n}\n#[cfg(test)] mod tests { #[test] fn test_usage() {} }\n",
        );
        assert!(violations
            .iter()
            .any(|d| d.message.contains("body is a single simple")));
    }

    #[test]
    fn test_usage_complex_match_body_is_clean() {
        let violations = run_body(
            "pub fn spin(w: &mut Widget) {\n    match w.count {\n        0 => { let a = 1; a }\n        _ => 2,\n    };\n}\n#[cfg(test)] mod tests { #[test] fn test_usage() {} }\n",
        );
        assert!(violations.is_empty());
    }

    #[test]
    fn test_usage_long_chain_body_is_clean() {
        let violations = run_body(
            "pub fn spin(w: &mut Widget) -> Vec<u32> { w.items.iter().map(|item| item.value * 2 + item.offset).filter(|v| *v > 10).map(|v| v * 3).collect::<Vec<u32>>() }\n#[cfg(test)] mod tests { #[test] fn test_usage() {} }\n",
        );
        assert!(violations.is_empty());
    }

    #[test]
    fn test_usage_over_width_format_body_is_clean() {
        let violations = run_body(
            "pub fn spin(w: &mut Widget) -> String { format!(\"the widget named {} carries a long diagnostic message that does not fit on one line of rustfmt output\", w.name) }\n#[cfg(test)] mod tests { #[test] fn test_usage() {} }\n",
        );
        assert!(!violations
            .iter()
            .any(|d| d.message.contains("body is a one-liner")));
    }
}
