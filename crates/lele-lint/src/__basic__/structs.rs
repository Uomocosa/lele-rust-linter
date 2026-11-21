use serde::Deserialize;

use crate::basic;
use crate::basic::enums::CodeDunderRole;
use crate::basic::enums::Origin;
use crate::basic::enums::Requirement;
use crate::basic::enums::RuleLevel;
use crate::basic::enums::TestingRole;
use crate::DunderPath;
use crate::Entry;

#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct LeleSection {
    #[serde(default)]
    pub project: ProjectSection,
    #[serde(default)]
    pub rules: std::collections::BTreeMap<String, std::collections::BTreeMap<String, RuleTable>>,
}

#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct ProjectSection {
    #[serde(default)]
    pub exclude: Vec<std::path::PathBuf>,
    #[serde(default)]
    pub e2e_test_dirs: Vec<std::path::PathBuf>,
    #[serde(default)]
    pub dunder_paths: Vec<DunderPath>,
    #[serde(default)]
    pub layout: LayoutSection,
    #[serde(default)]
    pub boundary: Vec<BoundaryEntry>,
    #[serde(default)]
    pub flat_dir: Vec<FlatDirEntry>,
    #[serde(default)]
    pub entry_points: Vec<EntryPoint>,
    #[serde(default)]
    pub inline_trait_methods: Vec<InlineTraitMethodsEntry>,
}

#[derive(Deserialize, Debug, Default)]
#[serde(deny_unknown_fields)]
pub struct RuleTable {
    pub level: Option<RuleLevel>,
    pub reason: Option<String>,
    #[serde(default)]
    pub exempt: Vec<toml::Table>,
}

#[derive(Deserialize, Debug, Clone, Default)]
#[serde(deny_unknown_fields)]
pub struct LayoutSection {
    #[serde(default)]
    pub roots: Vec<std::path::PathBuf>,
    #[serde(default)]
    pub methods_root: std::path::PathBuf,
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

#[derive(Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct FlatDirEntry {
    pub path: std::path::PathBuf,
    pub reason: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct InlineTraitMethodsEntry {
    #[serde(rename = "trait")]
    pub trait_path: String,
    pub methods: Vec<String>,
    pub reason: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct GenericExemptEntry {
    pub path: std::path::PathBuf,
    pub reason: String,
    pub replaced_by: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct CollectionWrapperEntry {
    #[serde(rename = "type")]
    pub type_name: String,
    pub file: std::path::PathBuf,
    pub reason: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct NoDelegatesEntry {
    #[serde(rename = "crate")]
    pub crate_path: std::path::PathBuf,
    pub reason: String,
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

#[derive(Debug, Clone)]
pub struct RuleSnapshot {
    pub name: String,
    pub code: String,
    pub namespace: String,
    pub is_locked: bool,
}

#[derive(Debug, Clone, Default)]
pub struct SyncOutcome {
    pub missing: Vec<String>,
    pub refreshed: Vec<String>,
    pub is_header_added: bool,
}

#[derive(Debug, Clone, Default)]
pub struct ConfigReportRow {
    pub code: String,
    pub name: String,
    pub namespace: String,
    pub locked: String,
    pub level: String,
    pub exempts: String,
    pub reason: String,
}

#[derive(Debug, Clone, Copy)]
pub struct ExampleFile {
    pub path: &'static str,
    pub source: &'static str,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DirFacts {
    pub origin: Origin,
    pub testing_role: Option<TestingRole>,
    pub code_dunder: Option<CodeDunderRole>,
    pub is_in_container: bool,
    pub is_in_tests: bool,
    pub is_in_flat_dir: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct RuleRelation {
    pub defers_to: &'static str,
    pub context: &'static str,
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
