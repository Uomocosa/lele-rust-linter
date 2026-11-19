use crate::scan;

pub fn missing_subject<'a>(
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
    use super::missing_subject;
    use crate::scan;
    use crate::testing;

    #[test]
    fn test_usage() {
        let subjects = vec![String::from("setup_ui")];
        let copy = [testing::preview(&["spawn_status"])];
        let real = [testing::preview(&["setup_ui"])];
        assert!(missing_subject(&copy, scan::PreviewKind::Png, &subjects).is_some());
        assert!(missing_subject(&real, scan::PreviewKind::Png, &subjects).is_none());
        assert!(missing_subject(&copy, scan::PreviewKind::Mp4, &subjects).is_none());
    }
}
