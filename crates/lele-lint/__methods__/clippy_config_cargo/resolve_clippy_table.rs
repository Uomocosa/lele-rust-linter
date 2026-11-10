use crate::rules;
use crate::Project;
use std::path::PathBuf;

pub fn resolve_clippy_table(
    value: &toml::Value,
    project: &Project,
    cargo_path: &PathBuf,
) -> Option<toml::value::Table> {
    if let Some(table) = value
        .get("lints")
        .and_then(|v| v.get("clippy"))
        .and_then(|v| v.as_table())
    {
        return Some(table.clone());
    }
    if let Some(workspace_table) = value
        .get("workspace")
        .and_then(|v| v.get("lints"))
        .and_then(|v| v.get("clippy"))
        .and_then(|v| v.as_table())
    {
        return Some(workspace_table.clone());
    }
    if rules::clippy_config_cargo::ClippyConfigCargo::inherits_workspace_lints(value) {
        if let Some(parent) =
            rules::clippy_config_cargo::ClippyConfigCargo::find_workspace_root(project)
        {
            let ws_cargo = parent.join("Cargo.toml");
            if ws_cargo != *cargo_path {
                if let Ok(ws_content) = std::fs::read_to_string(&ws_cargo) {
                    if let Ok(ws_value) = ws_content.parse::<toml::Value>() {
                        if let Some(ws_table) = ws_value
                            .get("workspace")
                            .and_then(|v| v.get("lints"))
                            .and_then(|v| v.get("clippy"))
                            .and_then(|v| v.as_table())
                        {
                            return Some(ws_table.clone());
                        }
                    }
                }
            }
        }
    }
    None
}

// no test_usage necessary
