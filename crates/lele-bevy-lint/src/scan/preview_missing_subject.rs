use crate::scan;

#[must_use]
pub fn preview_missing_subject<'a>(
    previews: &'a [scan::Preview],
    kind: scan::PreviewKind,
    subjects: &[String],
) -> Option<&'a scan::Preview> {
    let mut of_kind = previews.iter().filter(|preview| preview.kind == kind);
    let first = of_kind.clone().next()?;
    let is_rendered = of_kind.any(|preview| {
        preview
            .scene
            .build_uses
            .iter()
            .any(|used| subjects.contains(used))
    });
    (!is_rendered).then_some(first)
}

#[cfg(test)]
mod tests {
    use super::preview_missing_subject;
    use crate::scan;

    fn preview(uses: &[&str]) -> scan::Preview {
        scan::Preview {
            name: String::from("root_ui_png_preview"),
            kind: scan::PreviewKind::Png,
            line: 3,
            is_test: true,
            harness: scan::HarnessCall::Checked,
            scene: scan::SceneUsage {
                build_uses: uses.iter().map(|used| (*used).to_string()).collect(),
                state_count: 1,
                uncentered_kinds: Vec::new(),
            },
        }
    }

    #[test]
    fn test_usage() {
        let subjects = vec![String::from("setup_ui")];
        let copy = [preview(&["spawn_status"])];
        let real = [preview(&["setup_ui"])];
        assert!(preview_missing_subject(&copy, scan::PreviewKind::Png, &subjects).is_some());
        assert!(preview_missing_subject(&real, scan::PreviewKind::Png, &subjects).is_none());
        assert!(preview_missing_subject(&copy, scan::PreviewKind::Mp4, &subjects).is_none());
    }
}
