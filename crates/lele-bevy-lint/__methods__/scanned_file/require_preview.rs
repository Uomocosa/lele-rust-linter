use lele_lint::RuleViolation;

use crate::scan;

#[must_use]
pub fn require_preview(
    file: &scan::ScannedFile,
    kind: scan::PreviewKind,
    missing: impl FnOnce() -> RuleViolation,
) -> Option<RuleViolation> {
    let has_preview = file
        .collect_previews()
        .iter()
        .any(|preview| preview.kind == kind);
    (!has_preview).then(missing)
}

#[cfg(test)]
mod tests {
    use super::require_preview;
    use crate::scan;
    use crate::testing;

    #[test]
    fn test_usage() {
        let file = syn::parse_str("pub fn setup() {}").unwrap();
        let found = require_preview(
            &scan::ScannedFile(&file),
            scan::PreviewKind::Png,
            testing::preview_violation,
        );
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
        assert!(
            require_preview(
                &scan::ScannedFile(&file),
                scan::PreviewKind::Png,
                testing::preview_violation
            )
            .is_none()
        );
    }
}
