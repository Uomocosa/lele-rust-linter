use crate::Category;
use crate::ExampleFile;
use crate::Project;
use crate::Rule;
use crate::RuleDoc;
use crate::RuleViolation;
use atomic_delegate_macros::atomic_delegate;
use atomic_delegate_macros::atomic_delegates;
use std::path::PathBuf;

pub struct ClippyConfigCargo;

impl ClippyConfigCargo {
    pub const NAME: &'static str = "clippy_config_cargo";
    pub const CODE: &'static str = "LELE028";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Config,
        summary: "`Cargo.toml` has `[lints.clippy]` with `pedantic`/`nursery` denied at priority -1 and the 13 required deny lints (or inherits them from the workspace).",
        why: "Every crate gets the same strict baseline, so no crate silently allows unwraps, panics or lossy casts.",
        bad: &[
            ExampleFile {
                path: "Cargo.toml",
                source: r#"[package]
name = "example"
version = "0.1.0"
edition = "2021"
"#,
            },
        ],
        good: &[
            ExampleFile {
                path: "Cargo.toml",
                source: r#"[package]
name = "example"
version = "0.1.0"
edition = "2021"

[lints.clippy]
pedantic = { level = "deny", priority = -1 }
nursery = { level = "deny", priority = -1 }
unwrap_used = "deny"
expect_used = "deny"
indexing_slicing = "deny"
arithmetic_side_effects = "deny"
unreachable = "deny"
unimplemented = "deny"
unchecked_time_subtraction = "deny"
todo = "deny"
string_slice = "deny"
panic_in_result_fn = "deny"
panic = "deny"
exit = "deny"
as_conversions = "deny"
"#,
            },
        ],
    };
}

#[rustfmt::skip]
impl Rule for ClippyConfigCargo {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(ClippyConfigCargo)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl ClippyConfigCargo {
    pub fn register(rules: &mut Vec<Box<dyn Rule>>) {
        rules.push(Box::new(ClippyConfigCargo));
    }
    pub(crate) fn inherits_workspace_lints(value: &toml::Value) -> bool {
        value
            .get("lints")
            .and_then(|v| v.get("workspace"))
            .and_then(|v| v.as_bool())
            == Some(true)
    }
    pub(crate) fn is_deny_with_priority(value: &toml::Value, require_priority: bool) -> bool {
        match value {
            toml::Value::String(s) => s == "deny",
            toml::Value::Table(t) => {
                let level_ok = t.get("level").and_then(|v| v.as_str()) == Some("deny");
                if !level_ok {
                    return false;
                }
                if require_priority {
                    match t.get("priority").and_then(|v| v.as_integer()) {
                        Some(-1) => true,
                        _ => false,
                    }
                } else {
                    true
                }
            }
            _ => false,
        }
    }
}

#[atomic_delegates]
impl ClippyConfigCargo {
    pub(crate) fn resolve_clippy_table(
        value: &toml::Value,
        project: &Project,
        cargo_path: &PathBuf,
    ) -> Option<toml::value::Table> {
    }
    pub(crate) fn find_workspace_root(project: &Project) -> Option<PathBuf> {}
}

// no test_usage necessary
