use lele_lint::Project;
use lele_lint::RuleViolation;

use crate::rules;
use crate::scan;

pub fn check(
    _self: &rules::preview_routing::PreviewRouting,
    project: &Project,
) -> Vec<RuleViolation> {
    let mut violations = Vec::new();
    for source in project.sources() {
        for preview in scan::ScannedFile(source.file).collect_previews() {
            for message in rules::preview_routing::PreviewRouting::problems(&preview) {
                violations.push(scan::BevyProject(project).violation(
                    source.origin,
                    source.relative_path,
                    preview.line,
                    rules::preview_routing::PreviewRouting::CODE,
                    message,
                ));
            }
        }
    }
    violations
}

#[cfg(test)]
mod tests {
    use super::check;
    use crate::rules;
    use lele_lint::Project;
    use std::path::PathBuf;

    fn messages(source: &str) -> Vec<String> {
        let mut project = Project::default();
        project
            .parsed_files
            .insert(PathBuf::from("ui/root.rs"), syn::parse_str(source).unwrap());
        check(&rules::preview_routing::PreviewRouting, &project)
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
