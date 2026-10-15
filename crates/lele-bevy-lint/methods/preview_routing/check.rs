use lele_lint::Project;
use lele_lint::RuleViolation;

use crate::checkers;
use crate::scan;

pub fn check(
    _self: &checkers::preview_routing::PreviewRouting,
    project: &Project,
) -> Vec<RuleViolation> {
    let mut violations = Vec::new();
    for source in project.sources() {
        for preview in scan::collect_previews(source.file) {
            for message in problems(&preview) {
                violations.push(scan::site_violation(
                    project,
                    source.origin,
                    source.relative_path,
                    preview.line,
                    checkers::preview_routing::PreviewRouting::CODE,
                    message,
                ));
            }
        }
    }
    violations
}

// needed helper: every way one preview fn fails to run, render or check its result
fn problems(preview: &scan::Preview) -> Vec<String> {
    let mut found = Vec::new();
    if !preview.is_test {
        found.push(format!(
            "`fn {}` is named like a preview but has no `#[test]`, so it never runs",
            preview.name
        ));
    }
    match preview.harness {
        scan::HarnessCall::Missing => found.push(format!(
            "`fn {}` is a preview test but does not call `lele_bevy_preview::run(...)`; route it through the harness so an empty frame fails",
            preview.name
        )),
        scan::HarnessCall::Unchecked => found.push(format!(
            "`fn {}` discards the `lele_bevy_preview::run(...)` result; end it with `.expect(\"preview\")` (or `?`) so `NoVisual`/`StaticClip` fail the test",
            preview.name
        )),
        scan::HarnessCall::Checked => {}
    }
    found
}

#[cfg(test)]
mod tests {
    use super::check;
    use crate::checkers;
    use lele_lint::Project;
    use std::path::PathBuf;

    fn messages(source: &str) -> Vec<String> {
        let mut project = Project::default();
        project
            .parsed_files
            .insert(PathBuf::from("ui/root.rs"), syn::parse_str(source).unwrap());
        check(&checkers::preview_routing::PreviewRouting, &project)
            .into_iter()
            .map(|violation| violation.message)
            .collect()
    }

    #[test]
    fn test_usage() {
        let found = messages(
            "#[cfg(test)]
             mod tests {
                 #[test]
                 fn spawn_root_ui_png_preview() { assert!(true); }
             }",
        );
        assert_eq!(found.len(), 1);
        assert!(found[0].contains("does not call"));
    }

    #[test]
    fn test_usage_discarded_result_and_missing_test_attr() {
        let found = messages(
            "#[cfg(test)]
             mod tests {
                 use lele_bevy_preview::run;
                 fn spawn_root_ui_png_preview() { let _ = run(&s, &c, \"x\"); }
             }",
        );
        assert_eq!(found.len(), 2);
        assert!(found.iter().any(|message| message.contains("no `#[test]`")));
        assert!(found.iter().any(|message| message.contains("discards")));
    }

    #[test]
    fn test_usage_checked_test_is_clean() {
        let found = messages(
            "#[cfg(test)]
             mod tests {
                 #[test]
                 fn spawn_root_ui_png_preview() {
                     lele_bevy_preview::run(&s, &c, \"x\").expect(\"preview\");
                 }
             }",
        );
        assert_eq!(found, Vec::<String>::new());
    }
}
