use crate::scan;

pub(crate) fn preview(uses: &[&str]) -> scan::Preview {
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

#[cfg(test)]
mod tests {
    use super::preview;
    use crate::scan;

    #[test]
    fn test_usage() {
        let built = preview(&["setup_ui"]);
        assert_eq!(built.scene.build_uses, vec!["setup_ui".to_string()]);
        assert!(matches!(built.kind, scan::PreviewKind::Png));
    }
}
