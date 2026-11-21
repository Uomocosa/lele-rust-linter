use atomic_delegate_macros::atomic_delegates;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("no Cargo.toml found in {} or any parent directory", .0.display())]
    NoCargoRoot(std::path::PathBuf),

    #[error("src/ directory not found at {}", .0.display())]
    NoSrcDirectory(std::path::PathBuf),

    #[error("scan folder not found or not a directory: {}", .0.display())]
    NoScanFolder(std::path::PathBuf),

    #[error("methods layout requested but no `__methods__/` directory was found")]
    NoMethodsDir,

    #[error("filesystem error")]
    WalkDir(#[from] walkdir::Error),

    #[error("config error")]
    Config(#[from] toml::de::Error),

    #[error("I/O error")]
    Io(#[from] std::io::Error),

    #[error("dunder path `{}` must end in `__<name>__`", .0.display())]
    NotDunder(std::path::PathBuf),

    #[error("rustfmt could not be run in {}", .root.display())]
    RustfmtStart {
        root: std::path::PathBuf,
        source: std::io::Error,
    },

    #[error("rustfmt rejected the placement check: {stderr}")]
    RustfmtFailed { stderr: String },

    #[error("invalid `exempt` entry for rule `{rule}`")]
    BadRuleExempt {
        rule: String,
        #[source]
        source: toml::de::Error,
    },

    #[error("rustfmt output could not be parsed")]
    RustfmtOutput(#[source] syn::Error),

    #[error("rustfmt output lacks a placement answer for a method")]
    RustfmtMissing,

    #[error("lele.toml: no rule `{name}` in namespace `{namespace}`")]
    UnknownRule { namespace: String, name: String },

    #[error("lele.toml: `{name}` ({code}) is a LELECORE rule and cannot be turned off; exempt a path instead")]
    LockedRuleTurnedOff { name: String, code: String },

    #[error("lele.toml: `{name}` is turned off without a `reason`")]
    SettingWithoutReason { name: String },

    #[error("lele.toml: exemption in `{name}` is replaced by unknown rule `{replaced_by}`")]
    UnknownReplacement { name: String, replaced_by: String },

    #[error("lele.toml: `{name}` has an exemption without a `reason`")]
    ExemptWithoutReason { name: String },

    #[error("lele.toml sync failed")]
    SyncParse {
        #[source]
        source: toml_edit::TomlError,
    },

    #[error("prepare failed")]
    Prepare(#[source] Box<dyn std::error::Error + Send + Sync>),
}

#[atomic_delegates]
impl Error {
    pub fn with_causes(error: &dyn std::error::Error) -> String {}
}

// no test_usage necessary
