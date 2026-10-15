use lele_lint::RuleViolation;

use crate::scan;

#[must_use]
pub fn require_preview(
    file: &syn::File,
    kind: scan::PreviewKind,
    missing: impl FnOnce() -> RuleViolation,
) -> Option<RuleViolation> {
    let has_preview = scan::collect_previews(file)
        .iter()
        .any(|preview| preview.kind == kind);
    (!has_preview).then(missing)
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use lele_lint::Origin;
    use lele_lint::Project;

    use super::require_preview;
    use crate::scan;

    fn missing() -> lele_lint::RuleViolation {
        scan::site_violation(
            &Project::default(),
            Origin::Src,
            Path::new("a.rs"),
            1,
            "X",
            String::from("no preview"),
        )
    }

    #[test]
    fn test_usage() {
        let file = syn::parse_str("pub fn setup() {}").unwrap();
        let found = require_preview(&file, scan::PreviewKind::Png, missing);
        assert_eq!(found.map(|violation| violation.code), Some("X"));
    }

    #[test]
    fn test_usage_existing_preview_satisfies() {
        let file = syn::parse_str(
            "#[cfg(test)] mod tests {
                 #[test]
                 fn root_ui_png_preview() {}
             }",
        )
        .unwrap();
        assert!(require_preview(&file, scan::PreviewKind::Png, missing).is_none());
    }
}
