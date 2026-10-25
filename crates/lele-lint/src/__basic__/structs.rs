use serde::Deserialize;

use crate::basic;
use crate::basic::enums::Requirement;
use crate::DunderPath;
use crate::Entry;

#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct LeleSection {
    #[serde(default)]
    pub lint: LeleTomlLintSections,
    #[serde(default)]
    pub boundary: Vec<BoundaryEntry>,
    #[serde(default, rename = "config")]
    pub enforce_config: Option<toml::Value>,
}

#[derive(Deserialize)]
pub struct LeleToml {
    #[serde(default)]
    pub lele: LeleSection,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct BoundaryEntry {
    pub name: String,
    pub why: String,
    pub folders: Vec<std::path::PathBuf>,
    #[serde(default)]
    pub cannot_use: Vec<String>,
    #[serde(default)]
    pub require: Option<Requirement>,
}

#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct LeleTomlLintSections {
    #[serde(default)]
    pub dunder_paths: Vec<DunderPath>,
    #[serde(default)]
    pub clippy_allow_whitelist: Vec<AllowWhitelistEntry>,
    #[serde(default)]
    pub entry_points: Vec<EntryPoint>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct EntryPoint {
    pub file: std::path::PathBuf,
    #[serde(rename = "fn")]
    pub function: String,
    pub reason: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct AllowWhitelistEntry {
    pub allow: String,
    pub file: std::path::PathBuf,
    pub reason: String,
}

#[derive(Debug, Clone)]
pub struct ParseFailure {
    pub path: std::path::PathBuf,
    pub cause: String,
}

#[derive(Default)]
pub struct ScannedDir {
    pub entries: Vec<Entry>,
    pub parsed_files: std::collections::HashMap<std::path::PathBuf, syn::File>,
    pub parse_failures: Vec<ParseFailure>,
}

#[derive(Debug, Clone)]
pub struct ModDecl {
    pub name: String,
    pub is_public: bool,
    pub cfg: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Reexport {
    pub segments: Vec<String>,
    pub is_glob: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct ExampleFile {
    pub path: &'static str,
    pub source: &'static str,
}

#[derive(Debug, Clone, Copy)]
pub struct RuleDoc {
    pub category: basic::enums::Category,
    pub summary: &'static str,
    pub why: &'static str,
    pub fix: &'static str,
    pub bad: &'static [ExampleFile],
    pub good: &'static [ExampleFile],
}
