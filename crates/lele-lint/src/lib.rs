mod common;
pub mod common_args;
pub mod rules;
pub mod sync_args;
pub use sync_args::SyncArgs;
#[path = "__traits__/mod.rs"]
pub mod traits;
pub use common_args::CommonArgs;
pub mod config;
pub use config::Config;
pub mod config_sync;
pub use config_sync::ConfigSync;
#[path = "__basic__/mod.rs"]
mod basic;
pub use basic::enums::Category;
pub use basic::enums::CodeDunderRole;
pub(crate) use basic::enums::DunderKind;
pub use basic::enums::EntryKind;
pub(crate) use basic::enums::ImplItemGroup;
pub use basic::enums::Location;
pub use basic::enums::Origin;
pub use basic::enums::PlacementKind;
pub use basic::enums::Requirement;
pub use basic::enums::RuleLevel;
pub use basic::enums::TestingRole;
pub use basic::structs::{
    AllowWhitelistEntry, BoundaryEntry, CollectionWrapperEntry, ConfigReportRow, DirFacts,
    EntryPoint, ExampleFile, FlatDirEntry, GenericExemptEntry, InlineTraitMethodsEntry,
    LayoutSection, LeleSection, LeleToml, ModDecl, NoDelegatesEntry, ParseFailure, ProjectSection,
    Reexport, RuleDoc, RuleRelation, RuleSnapshot, RuleTable, ScannedDir, SyncOutcome,
};
pub use basic::type_aliases::ModuleInfoMap;
pub use basic::type_aliases::RustfmtKey;
pub mod dunder_path;
pub use dunder_path::DunderPath;
pub mod entry;
pub use entry::Entry;
pub mod error;
pub use error::Error;
pub mod error_format;
pub use error_format::ErrorFormat;
pub mod fn_body;
pub use fn_body::FnBody;
pub mod impl_block;
pub use impl_block::ImplBlock;
pub mod impl_method;
pub use impl_method::ImplMethod;
pub mod identifier;
pub use identifier::Identifier;
pub mod item_marker;
pub mod lint_session;
pub mod linter;
pub use item_marker::ItemMarker;
pub use lint_session::LintSession;
pub use linter::Linter;
pub mod module_info;
pub use module_info::ModuleInfo;
#[path = "../__methods__/mod.rs"]
mod methods;
pub mod parsed_source;
pub use parsed_source::ParsedSource;
pub mod path_exemption;
pub use path_exemption::PathExemption;
pub mod project;
pub use project::Project;
pub mod visibility;
pub use visibility::Visibility;
pub mod raw_code;
pub use raw_code::RawCode;
pub mod relaxed_rule;
pub use relaxed_rule::RelaxedRule;
pub mod rule_violation;
pub use rule_violation::RuleViolation;

#[cfg(test)]
pub(crate) mod testing;
