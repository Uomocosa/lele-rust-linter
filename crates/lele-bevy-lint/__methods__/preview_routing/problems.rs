use crate::scan;

pub fn problems(preview: &scan::Preview) -> Vec<String> {
    let mut found = Vec::new();
    if !preview.is_test {
        found.push(format!(
            "`fn {}` is named like a preview but has no `#[test]`, so it never runs — add `#[test]`",
            preview.name
        ));
    }
    match preview.harness {
        scan::HarnessCall::Missing => found.push(format!(
            "`fn {}` is a preview test but does not call the preview harness (`scene.run(...)` or `lele_bevy_preview::run(...)`) — route it through the harness so an empty frame fails",
            preview.name
        )),
        scan::HarnessCall::Unchecked => found.push(format!(
            "`fn {}` discards the harness `run(...)` result — end it with `.expect(\"preview\")` (or `?`) so `NoVisual`/`StaticClip` fail the test",
            preview.name
        )),
        scan::HarnessCall::Checked => {}
    }
    found
}

#[cfg(test)]
mod tests {
    use super::problems;
    use crate::scan;

    #[test]
    fn test_usage() {
        let preview = scan::Preview {
            name: "x".to_string(),
            line: 1,
            is_test: false,
            harness: scan::HarnessCall::Missing,
            kind: scan::PreviewKind::Png,
            scene: scan::SceneUsage::default(),
        };
        assert_ne!(problems(&preview).len(), 0);
    }
}
