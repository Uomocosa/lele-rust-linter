use std::path::PathBuf;

use lele_lint::Linter;

/// `RULES.md` is generated from the rule docs and must not drift from them.
/// Regenerate it with `just docs`.
#[test]
fn rules_md_is_up_to_date() -> Result<(), Box<dyn std::error::Error>> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("RULES.md");
    let actual = Linter::default().rules_markdown();
    let expected = std::fs::read_to_string(&path).unwrap_or_default();
    if expected == actual {
        return Ok(());
    }
    Err("RULES.md is stale — regenerate with `just docs`".into())
}
