use std::collections::BTreeSet;
use std::path::PathBuf;

use lele_lint::Linter;
use lele_lint::Project;
use rstest::rstest;
use serde::Deserialize;

#[derive(Deserialize)]
struct Expected {
    violations: Vec<ExpectedViolation>,
}

#[derive(Deserialize)]
struct ExpectedViolation {
    file: String,
    code: String,
}

fn test_crate_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../lele-lint-test-crates/lele-lint")
        .join(name)
}

fn findings(name: &str) -> Result<BTreeSet<(String, String)>, Box<dyn std::error::Error>> {
    let root = test_crate_path(name);
    let mut project = Project::try_from_start_dir_and_scan_folders(Some(&root), None)?;
    let config = project.load_config().unwrap_or_default();
    project.apply_layout(&config)?;
    Ok(Linter::default()
        .rules
        .iter()
        .flat_map(|rule| rule.check(&project))
        .map(|violation| {
            let file = violation
                .file
                .strip_prefix(&root)
                .unwrap_or(&violation.file);
            (file.display().to_string(), violation.code.to_string())
        })
        .collect())
}

fn expected(name: &str) -> Result<BTreeSet<(String, String)>, Box<dyn std::error::Error>> {
    let text = std::fs::read_to_string(test_crate_path(name).join("expected.toml"))?;
    let parsed: Expected = toml::from_str(&text)?;
    Ok(parsed
        .violations
        .into_iter()
        .map(|violation| (violation.file, violation.code))
        .collect())
}

/// Every test crate reports exactly the `(file, code)` pairs in its `expected.toml`.
#[rstest]
#[case("compliant_crate")]
#[case("violation_crate")]
#[case("methods_crate")]
#[case("methods_violation_crate")]
fn test_crate_matches_expected(#[case] name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let found = findings(name)?;
    let wanted = expected(name)?;
    let missing: Vec<_> = wanted.difference(&found).collect();
    let unexpected: Vec<_> = found.difference(&wanted).collect();
    if missing.is_empty() && unexpected.is_empty() {
        return Ok(());
    }
    Err(format!("{name}:\nmissing: {missing:#?}\nunexpected: {unexpected:#?}").into())
}

#[test]
fn test_multi_root_layout_and_crate_local_boundary() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempfile::tempdir()?;
    let root = dir.path();
    std::fs::write(
        root.join("Cargo.toml"),
        "[package]\nname = \"multi_root_probe\"\nversion = \"0.0.0\"\n",
    )?;
    std::fs::write(
        root.join("lele.toml"),
        "[lele.layout]\nroots = [\"src\", \"ui\"]\nmethods_root = \"__methods__\"\n\n[[lele.boundary]]\nname = \"src stays free of ui\"\nwhy = \"ui is an adapter\"\nfolders = [\"src\"]\ncannot_use = [\"crate::ui\"]\n",
    )?;
    std::fs::create_dir_all(root.join("src"))?;
    std::fs::create_dir_all(root.join("ui"))?;
    for sub in ["src/widget", "src/gadget", "ui/gadget"] {
        std::fs::create_dir_all(root.join("__methods__").join(sub))?;
    }
    std::fs::write(
        root.join("src").join("lib.rs"),
        "pub mod gadget;\npub mod widget;\n#[path = \"../ui/mod.rs\"]\npub mod ui;\n",
    )?;
    std::fs::write(
        root.join("src").join("widget.rs"),
        "use crate::ui;\n\npub struct Widget;\n\n#[atomic_delegates]\nimpl Widget {\n    pub fn spin(&self) {}\n}\n",
    )?;
    std::fs::write(
        root.join("src").join("gadget.rs"),
        "pub struct Gadget;\n\n#[atomic_delegates]\nimpl Gadget {\n    pub fn fetch(&self) {}\n}\n",
    )?;
    std::fs::write(
        root.join("ui").join("mod.rs"),
        "pub mod thing;\npub mod user;\npub mod widget;\n",
    )?;
    std::fs::write(root.join("ui").join("thing.rs"), "pub struct Thing;\n")?;
    std::fs::write(
        root.join("ui").join("user.rs"),
        "use crate::ui::Thing;\n\npub struct User;\n",
    )?;
    std::fs::write(root.join("ui").join("widget.rs"), "pub struct Widget;\n")?;
    std::fs::write(
        root.join("__methods__").join("mod.rs"),
        "#[path = \"src/mod.rs\"]\nmod src_root;\npub use src_root::*;\n#[path = \"ui/mod.rs\"]\nmod ui_root;\npub use ui_root::*;\n",
    )?;
    std::fs::write(
        root.join("__methods__").join("src").join("mod.rs"),
        "pub mod gadget;\npub mod widget;\n",
    )?;
    std::fs::write(root.join("__methods__").join("ui").join("mod.rs"), "")?;
    std::fs::write(
        root.join("__methods__")
            .join("src")
            .join("widget")
            .join("mod.rs"),
        "mod spin;\npub use spin::spin;\n",
    )?;
    std::fs::write(
        root.join("__methods__")
            .join("src")
            .join("gadget")
            .join("mod.rs"),
        "mod fetch;\npub use fetch::fetch;\n",
    )?;
    std::fs::write(
        root.join("__methods__")
            .join("src")
            .join("widget")
            .join("spin.rs"),
        "use crate::ui::Thing;\n\npub fn spin(widget: &Widget, thing: Thing) {\n    let _ = widget;\n    let _ = thing;\n}\n\n#[cfg(test)]\nmod tests {\n    use super::spin;\n    use crate::ui::Thing;\n    use crate::widget::Widget;\n\n    #[test]\n    fn test_usage() {\n        spin(&Widget, Thing);\n    }\n}\n",
    )?;
    std::fs::write(
        root.join("__methods__")
            .join("ui")
            .join("gadget")
            .join("mod.rs"),
        "// no test_usage necessary\n",
    )?;
    std::fs::write(
        root.join("__methods__")
            .join("ui")
            .join("gadget")
            .join("fetch.rs"),
        "use crate::gadget::Gadget;\n\npub fn fetch(gadget: &Gadget) {\n    let _ = gadget;\n    let _ = gadget;\n}\n\n#[cfg(test)]\nmod tests {\n    use super::fetch;\n    use crate::gadget::Gadget;\n\n    #[test]\n    fn test_usage() {\n        fetch(&Gadget);\n    }\n}\n",
    )?;
    let mut project = Project::try_from_start_dir_and_scan_folders(Some(root), None)?;
    let config = project.load_config().unwrap_or_default();
    project.apply_layout(&config)?;
    let violations: Vec<(String, String, String)> = Linter::default()
        .rules
        .iter()
        .flat_map(|rule| rule.check(&project))
        .map(|violation| {
            (
                violation.file.display().to_string(),
                violation.code.to_string(),
                violation.message,
            )
        })
        .collect();
    let has = |file_part: &str, code: &str, message_part: &str| {
        violations.iter().any(|(file, found, message)| {
            file.contains(file_part) && found == code && message.contains(message_part)
        })
    };
    if !has("src/widget.rs", "LELE033", "crate::ui") {
        return Err(format!(
            "expected LELE033 for `use crate::ui` in src/widget.rs, got {violations:#?}"
        )
        .into());
    }
    if !has("__methods__/src/widget/spin.rs", "LELE033", "crate::ui") {
        return Err(format!("expected LELE033 for `use crate::ui::Thing` in __methods__/src/widget/spin.rs, got {violations:#?}").into());
    }
    if violations
        .iter()
        .any(|(file, code, _)| file.contains("ui/user.rs") && code == "LELE033")
    {
        return Err(format!("expected no LELE033 inside ui/user.rs, got {violations:#?}").into());
    }
    if !has("ui/gadget/fetch.rs", "LELE030", "__methods__/src/gadget/") {
        return Err(format!("expected LELE030 naming `__methods__/src/gadget/` for the misplaced methods file, got {violations:#?}").into());
    }
    if !has("ui/widget.rs", "LELE030", "more than one source root") {
        return Err(format!(
            "expected LELE030 for duplicate `Widget` across roots, got {violations:#?}"
        )
        .into());
    }
    Ok(())
}

#[test]
fn test_layout_extra_roots_are_linted() -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempfile::tempdir()?;
    let root = dir.path();
    std::fs::write(
        root.join("Cargo.toml"),
        "[package]\nname = \"layout_probe\"\nversion = \"0.0.0\"\n",
    )?;
    std::fs::write(
        root.join("lele.toml"),
        "[lele.layout]\nroots = [\"src\", \"ui\"]\nmethods_root = \"__methods__\"\n",
    )?;
    std::fs::create_dir_all(root.join("src"))?;
    std::fs::create_dir_all(root.join("ui"))?;
    std::fs::create_dir_all(root.join("__methods__").join("foo"))?;
    std::fs::write(
        root.join("src").join("lib.rs"),
        "#[path = \"../ui/mod.rs\"]\npub mod ui;\n",
    )?;
    std::fs::write(root.join("ui").join("mod.rs"), "pub mod discovery;\n")?;
    std::fs::write(
        root.join("ui").join("discovery.rs"),
        "pub fn discover() -> u32 {\n    let first = 1;\n    let second = 2;\n    first + second\n}\n",
    )?;
    std::fs::write(
        root.join("__methods__").join("foo").join("bar.rs"),
        "pub fn bar() {\n    let first = 1;\n    let second = 2;\n    let _ = first + second;\n}\n",
    )?;
    std::fs::write(
        root.join("__methods__").join("mod.rs"),
        "// no test_usage necessary\n",
    )?;
    std::fs::write(
        root.join("__methods__").join("foo").join("mod.rs"),
        "// no test_usage necessary\n",
    )?;
    let mut project = Project::try_from_start_dir_and_scan_folders(Some(root), None)?;
    let config = project.load_config().unwrap_or_default();
    project.apply_layout(&config)?;
    let files: Vec<String> = Linter::default()
        .rules
        .iter()
        .flat_map(|rule| rule.check(&project))
        .map(|violation| violation.file.display().to_string())
        .collect();
    if !files.iter().any(|file| file.ends_with("ui/discovery.rs")) {
        return Err(format!("expected a violation in ui/discovery.rs, got {files:#?}").into());
    }
    if !files
        .iter()
        .any(|file| file.contains("__methods__") && file.ends_with("foo/bar.rs"))
    {
        return Err(
            format!("expected a violation in __methods__/foo/bar.rs, got {files:#?}").into(),
        );
    }
    Ok(())
}
