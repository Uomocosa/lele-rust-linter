use std::collections::BTreeSet;
use std::path::PathBuf;

use lele_lint::Config;
use lele_lint::Linter;
use lele_lint::Project;
use lele_lint::RelaxedRule;
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
#[case("stale_methods_crate")]
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

fn interactions_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../lele-lint-test-crates/lele-lint/interactions")
}

fn findings_with_config(
    root: &PathBuf,
    config_file: &PathBuf,
) -> Result<BTreeSet<(String, String)>, Box<dyn std::error::Error>> {
    let mut project = Project::try_from_start_dir_and_scan_folders(Some(root), None)?;
    let config = Config::try_from(config_file.as_path())?;
    project.apply_layout(&config)?;
    Ok(Linter::default()
        .rules
        .iter()
        .flat_map(|rule| rule.check(&project))
        .map(|violation| {
            let file = violation.file.strip_prefix(root).unwrap_or(&violation.file);
            (file.display().to_string(), violation.code.to_string())
        })
        .collect())
}

fn expected_at(path: &PathBuf) -> Result<BTreeSet<(String, String)>, Box<dyn std::error::Error>> {
    let text = std::fs::read_to_string(path)?;
    let parsed: Expected = toml::from_str(&text)?;
    Ok(parsed
        .violations
        .into_iter()
        .map(|violation| (violation.file, violation.code))
        .collect())
}

/// Every `interactions/*` crate lints to its `expected.toml` under every `lele*.toml`
/// config present (`lele.toml` pairs with `expected.toml`, `lele.<variant>.toml`
/// with `expected.<variant>.toml`).
#[test]
fn interaction_crates_match_expected() -> Result<(), Box<dyn std::error::Error>> {
    let root = interactions_root();
    let mut crates: Vec<PathBuf> = std::fs::read_dir(&root)?
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| path.is_dir())
        .collect();
    crates.sort();
    if crates.is_empty() {
        return Err("no interaction crates under interactions/".into());
    }
    for crate_dir in &crates {
        let mut configs: Vec<PathBuf> = std::fs::read_dir(crate_dir)?
            .filter_map(|entry| entry.ok().map(|entry| entry.path()))
            .filter(|path| {
                path.is_file()
                    && path
                        .file_name()
                        .is_some_and(|name| name.to_string_lossy().starts_with("lele"))
                    && path.extension().is_some_and(|ext| ext == "toml")
            })
            .collect();
        configs.sort();
        if configs.is_empty() {
            return Err(format!("{}: no lele*.toml config", crate_dir.display()).into());
        }
        for config_path in &configs {
            let config_name = config_path
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or("");
            let variant = config_name
                .strip_prefix("lele")
                .and_then(|rest| rest.strip_suffix(".toml"))
                .unwrap_or("");
            let expected_path = crate_dir.join(format!("expected{variant}.toml"));
            let found = findings_with_config(crate_dir, config_path)?;
            let wanted = expected_at(&expected_path)?;
            let missing: Vec<_> = wanted.difference(&found).collect();
            let unexpected: Vec<_> = found.difference(&wanted).collect();
            if !missing.is_empty() || !unexpected.is_empty() {
                return Err(format!(
                    "{} under {}:\nmissing: {missing:#?}\nunexpected: {unexpected:#?}",
                    crate_dir
                        .file_name()
                        .and_then(|name| name.to_str())
                        .unwrap_or("?"),
                    config_name,
                )
                .into());
            }
        }
    }
    Ok(())
}

/// Every declared `Rule::relations` entry has an `interactions/<rule>__<defers_to>/` crate proving it.
#[test]
fn every_rule_relation_has_an_interaction_crate() -> Result<(), Box<dyn std::error::Error>> {
    let root = interactions_root();
    let missing: Vec<String> = Linter::default()
        .rules
        .iter()
        .flat_map(|rule| {
            rule.relations()
                .iter()
                .map(move |relation| (rule.name().to_string(), relation.defers_to.to_string()))
        })
        .filter(|(name, defers_to)| !root.join(format!("{name}__{defers_to}")).is_dir())
        .map(|(name, defers_to)| format!("{name} × {defers_to}"))
        .collect();
    if missing.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "relations without an interaction crate:\n{}",
            missing.join("\n")
        )
        .into())
    }
}
/// The exemption crate: a `no_comments` exemption suppresses the comment in
/// `src/legacy`, while the `src/vanished` exemption suppresses nothing and is
/// reported as stale.
#[test]
fn exemption_crate_matches_expected() -> Result<(), Box<dyn std::error::Error>> {
    let name = "exemption_crate";
    let root = test_crate_path(name);
    let mut project = Project::try_from_start_dir_and_scan_folders(Some(&root), None)?;
    let config = project.load_config().unwrap_or_default();
    project.apply_layout(&config)?;
    let mut linter = Linter::default();
    let relaxed = linter.apply_rule_settings(&config)?;
    if relaxed.len() != 2 {
        return Err(format!("expected 2 relaxed entries, got {relaxed:#?}").into());
    }
    let first_is_replaced = matches!(
        &relaxed[0],
        RelaxedRule::Exempt {
            name,
            replaced_by: Some(replacement),
            ..
        } if name == "no_comments" && replacement == "test_usage"
    );
    if !first_is_replaced {
        return Err(format!("expected a replaced_by entry first, got {relaxed:#?}").into());
    }
    let found: BTreeSet<(String, String)> = linter
        .violations(&project)
        .into_iter()
        .map(|violation| {
            let file = violation
                .file
                .strip_prefix(&root)
                .unwrap_or(&violation.file);
            (file.display().to_string(), violation.code.to_string())
        })
        .collect();
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
        "[lele.project.layout]\nroots = [\"src\", \"ui\"]\nmethods_root = \"__methods__\"\n\n[[lele.project.boundary]]\nname = \"src stays free of ui\"\nwhy = \"ui is an adapter\"\nfolders = [\"src\"]\ncannot_use = [\"crate::ui\"]\n",
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
    if !has(
        "ui/gadget/fetch.rs",
        "LELECORE014",
        "__methods__/src/gadget/",
    ) {
        return Err(format!("expected LELECORE014 naming `__methods__/src/gadget/` for the misplaced methods file, got {violations:#?}").into());
    }
    if !has("ui/widget.rs", "LELECORE014", "more than one source root") {
        return Err(format!(
            "expected LELECORE014 for duplicate `Widget` across roots, got {violations:#?}"
        )
        .into());
    }
    Ok(())
}

#[test]
fn test_custom_methods_root_mount_visibility_follows_config(
) -> Result<(), Box<dyn std::error::Error>> {
    let dir = tempfile::tempdir()?;
    let root = dir.path();
    std::fs::write(
        root.join("Cargo.toml"),
        "[package]\nname = \"custom_methods_root_probe\"\nversion = \"0.0.0\"\n",
    )?;
    std::fs::write(
        root.join("lele.toml"),
        "[lele.project.layout]\nmethods_root = \"bodies\"\n",
    )?;
    std::fs::create_dir_all(root.join("bodies"))?;
    std::fs::create_dir_all(root.join("src"))?;
    std::fs::write(
        root.join("src").join("lib.rs"),
        "#[path = \"../bodies/mod.rs\"]\npub mod bodies;\n\npub mod methods;\n",
    )?;
    std::fs::write(
        root.join("bodies").join("mod.rs"),
        "// no test_usage necessary\n",
    )?;
    let mut project = Project::try_from_start_dir_and_scan_folders(Some(root), None)?;
    let config = project.load_config().unwrap_or_default();
    project.apply_layout(&config)?;
    let hits: Vec<(String, String)> = Linter::default()
        .rules
        .iter()
        .flat_map(|rule| rule.check(&project))
        .map(|violation| (violation.code.to_string(), violation.message))
        .collect();
    if !hits.iter().any(|(code, message)| {
        code == "LELECORE018" && message.contains("`pub mod bodies;` must be private")
    }) {
        return Err(format!(
            "expected LELECORE018 mount-visibility for `pub mod bodies;` under methods_root = \"bodies\", got {hits:#?}"
        )
        .into());
    }
    if hits
        .iter()
        .any(|(code, message)| code == "LELECORE018" && message.contains("`pub mod methods;`"))
    {
        return Err(format!(
            "expected no LELECORE018 for `pub mod methods;` when methods_root = \"bodies\", got {hits:#?}"
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
        "[lele.project.layout]\nroots = [\"src\", \"ui\"]\nmethods_root = \"__methods__\"\n",
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
