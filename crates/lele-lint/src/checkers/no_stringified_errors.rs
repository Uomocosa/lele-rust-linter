use crate::Category;
use crate::Checker;
use crate::ExampleFile;
use crate::Project;
use crate::RuleDoc;
use crate::RuleViolation;
use atomic_delegate_macros::atomic_delegate;

pub struct NoStringifiedErrors;

impl NoStringifiedErrors {
    pub const NAME: &'static str = "no_stringified_errors";
    pub const CODE: &'static str = "LELE039";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Types,
        summary: "A caught error is never stored as text in the crate's own error enum (`map_err`, `impl From`, or a helper taking `…Error`): keep it as a `#[source]`/`#[from]` field.",
        why: "Text throws away the error's type, its `source()` chain and `?` conversions; callers can only print it, never match or inspect it.",
        bad: &[
            ExampleFile {
                path: "src/lib.rs",
                source: r"mod error;
pub use error::Error;

mod config_text;
pub use config_text::ConfigText;
",
            },
            ExampleFile {
                path: "src/error.rs",
                source: r#"use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("cannot read {path}: {message}")]
    Read { path: PathBuf, message: String },
}
"#,
            },
            ExampleFile {
                path: "src/config_text.rs",
                source: r#"use std::path::Path;

use derive_more::Deref;

use crate::Error;

#[derive(Deref)]
pub struct ConfigText(pub String);

impl TryFrom<&Path> for ConfigText {
    type Error = Error;
    fn try_from(path: &Path) -> Result<Self, Error> {
        let text = std::fs::read_to_string(path).map_err(|error| Error::Read {
            path: path.to_path_buf(),
            message: error.to_string(),
        })?;
        Ok(Self(text))
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::ConfigText;

    #[test]
    fn test_usage() {
        assert!(ConfigText::try_from(Path::new("/nonexistent")).is_err());
    }
}
"#,
            },
        ],
        good: &[
            ExampleFile {
                path: "src/lib.rs",
                source: r"mod error;
pub use error::Error;

mod config_text;
pub use config_text::ConfigText;
",
            },
            ExampleFile {
                path: "src/error.rs",
                source: r#"use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("cannot read {path}")]
    Read {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
}
"#,
            },
            ExampleFile {
                path: "src/config_text.rs",
                source: r#"use std::path::Path;

use derive_more::Deref;

use crate::Error;

#[derive(Deref)]
pub struct ConfigText(pub String);

impl TryFrom<&Path> for ConfigText {
    type Error = Error;
    fn try_from(path: &Path) -> Result<Self, Error> {
        let text = std::fs::read_to_string(path).map_err(|source| Error::Read {
            path: path.to_path_buf(),
            source,
        })?;
        Ok(Self(text))
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::ConfigText;

    #[test]
    fn test_usage() {
        assert!(ConfigText::try_from(Path::new("/nonexistent")).is_err());
    }
}
"#,
            },
        ],
    };
}

#[rustfmt::skip]
impl Checker for NoStringifiedErrors {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(NoStringifiedErrors)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl NoStringifiedErrors {
    pub fn register(checkers: &mut Vec<Box<dyn Checker>>) {
        checkers.push(Box::new(NoStringifiedErrors));
    }
}

// no test_usage necessary
