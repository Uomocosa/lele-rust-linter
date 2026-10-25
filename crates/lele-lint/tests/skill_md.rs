use std::path::PathBuf;

use lele_lint::Linter;

const MAX_SKILL_LINES: usize = 250;

/// `skills/lele-lint-rules-rs/SKILL.md` is generated from the rule docs and must not drift from
/// them. Regenerate it with `just docs`.
#[test]
fn skill_md_is_up_to_date() -> Result<(), Box<dyn std::error::Error>> {
    let path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../skills/lele-lint-rules-rs/SKILL.md");
    let actual = Linter::default().skill_markdown();
    let expected = std::fs::read_to_string(&path).unwrap_or_default();
    if expected == actual {
        return Ok(());
    }
    Err("SKILL.md is stale — regenerate with `just docs`".into())
}

/// Small agents read the whole skill; keep it short enough to read in one go.
#[test]
fn skill_md_fits_line_budget() {
    let lines = Linter::default().skill_markdown().lines().count();
    assert!(
        lines <= MAX_SKILL_LINES,
        "SKILL.md has {lines} lines, budget is {MAX_SKILL_LINES}"
    );
}
