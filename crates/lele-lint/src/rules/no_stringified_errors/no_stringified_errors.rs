use crate::Category;
use crate::ExampleFile;
use crate::ItemMarker;
use crate::Project;
use crate::Rule;
use crate::RuleDoc;
use crate::RuleViolation;
use atomic_delegate_macros::atomic_delegate;
use std::collections::HashSet;

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
impl Rule for NoStringifiedErrors {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(NoStringifiedErrors)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl NoStringifiedErrors {
    pub fn register(rules: &mut Vec<Box<dyn Rule>>) {
        rules.push(Box::new(NoStringifiedErrors));
    }
    pub(crate) fn own_error_enums(project: &Project) -> HashSet<String> {
        project
            .content_sources()
            .flat_map(|source| source.file.items.iter())
            .filter_map(|item| match item {
                syn::Item::Enum(item_enum)
                    if item_enum
                        .attrs
                        .iter()
                        .any(|attr| ItemMarker(attr).derives_any_of(&["Error"])) =>
                {
                    Some(item_enum.ident.to_string())
                }
                _ => None,
            })
            .collect()
    }
    pub(crate) fn mentions_name(tokens: proc_macro2::TokenStream, name: &str) -> bool {
        tokens.into_iter().any(|token| match token {
            proc_macro2::TokenTree::Ident(ident) => ident == name,
            proc_macro2::TokenTree::Literal(literal) => {
                let text = literal.to_string();
                text.contains(&format!("{{{name}}}")) || text.contains(&format!("{{{name}:"))
            }
            proc_macro2::TokenTree::Group(group) => Self::mentions_name(group.stream(), name),
            proc_macro2::TokenTree::Punct(_) => false,
        })
    }
    pub(crate) fn names_an_error_type(ty: &syn::Type) -> bool {
        match ty {
            syn::Type::Reference(reference) => Self::names_an_error_type(&reference.elem),
            syn::Type::Path(path) => path
                .path
                .segments
                .last()
                .is_some_and(|segment| segment.ident.to_string().ends_with("Error")),
            _ => false,
        }
    }
    pub(crate) fn last_ident(ty: &syn::Type) -> Option<String> {
        match ty {
            syn::Type::Path(path) => path
                .path
                .segments
                .last()
                .map(|segment| segment.ident.to_string()),
            _ => None,
        }
    }
}

// no test_usage necessary
