use std::path::Path;

use lele_lint::Origin;
use lele_lint::Project;
use lele_lint::RuleViolation;

use crate::scan;

#[must_use]
pub fn require_ignored(
    project: &Project,
    origin: Origin,
    rel_path: &Path,
    preview: &scan::Preview,
    code: &'static str,
) -> Option<RuleViolation> {
    (!preview.is_ignored).then(|| {
        scan::site_violation(
            project,
            origin,
            rel_path,
            preview.line,
            code,
            format!(
                "`fn {}` must carry `#[ignore]` so the default test suite stays GPU-free",
                preview.name
            ),
        )
    })
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::require_ignored;
    use crate::scan;
    use lele_lint::Location;
    use lele_lint::Origin;
    use lele_lint::Project;

    fn preview(ignored: bool) -> scan::Preview {
        scan::Preview {
            name: String::from("root_ui_png_preview"),
            kind: scan::PreviewKind::Png,
            line: 4,
            is_ignored: ignored,
            is_routed_through_harness: true,
            idents: Vec::new(),
        }
    }

    #[test]
    fn test_usage() {
        let project = Project::default();
        let path = Path::new("a.rs");
        let violation = require_ignored(&project, Origin::Src, path, &preview(false), "E029");
        let violation = violation.unwrap();
        assert_eq!(violation.location, Location::Site { line: 4, col: 0 });
        assert!(violation.message.contains("root_ui_png_preview"));
        assert!(require_ignored(&project, Origin::Src, path, &preview(true), "E029").is_none());
    }
}
