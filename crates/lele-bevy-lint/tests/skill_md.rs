use std::path::PathBuf;

use lele_bevy_lint::BevyLinter;

/// `.opencode/skills/bevy-ui-preview/SKILL.md` is generated from the rule docs
/// and must not drift from them. Regenerate it with `just docs`.
#[test]
fn skill_md_is_up_to_date() -> Result<(), Box<dyn std::error::Error>> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../.opencode/skills/bevy-ui-preview/SKILL.md");
    let actual = BevyLinter::default().skill_markdown();
    let expected = std::fs::read_to_string(&path).unwrap_or_default();
    if expected == actual {
        return Ok(());
    }
    Err("SKILL.md is stale — regenerate with `just docs`".into())
}
