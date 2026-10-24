use crate::Category;
use crate::ExampleFile;
use crate::Project;
use crate::Rule;
use crate::RuleDoc;
use crate::RuleViolation;
use atomic_delegate_macros::atomic_delegate;
use atomic_delegate_macros::atomic_delegates;

pub struct ErrorMessageOmitsSource;

impl ErrorMessageOmitsSource {
    pub const NAME: &'static str = "error_message_omits_source";
    pub const CODE: &'static str = "LELE040";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Types,
        summary: "An `#[error(\"…\")]` message does not print its own source (the `#[from]`/`#[source]`/`source` field); use `#[error(transparent)]` to forward it instead.",
        why: "Error reporters walk the `source()` chain and print every cause, so a message that also prints the source shows it twice. A binary reports `error: cause: cause…` by walking `source()` once, at the top.",
        bad: &[
            ExampleFile {
                path: "src/lib.rs",
                source: r"mod error;
pub use error::Error;
",
            },
            ExampleFile {
                path: "src/error.rs",
                source: r#"#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("i/o error: {0}")]
    Io(#[from] std::io::Error),
}
"#,
            },
        ],
        good: &[
            ExampleFile {
                path: "src/lib.rs",
                source: r"mod error;
pub use error::Error;
",
            },
            ExampleFile {
                path: "src/error.rs",
                source: r#"#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("i/o error")]
    Io(#[from] std::io::Error),
}
"#,
            },
        ],
    };
}

#[rustfmt::skip]
impl Rule for ErrorMessageOmitsSource {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(ErrorMessageOmitsSource)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl ErrorMessageOmitsSource {
    pub fn register(rules: &mut Vec<Box<dyn Rule>>) {
        rules.push(Box::new(ErrorMessageOmitsSource));
    }
    pub(crate) fn source_field_name(fields: &syn::Fields) -> Option<String> {
        fields.iter().enumerate().find_map(|(index, field)| {
            let is_marked = field
                .attrs
                .iter()
                .any(|attr| attr.path().is_ident("from") || attr.path().is_ident("source"));
            let name = field
                .ident
                .as_ref()
                .map_or_else(|| index.to_string(), ToString::to_string);
            (is_marked || name == "source").then_some(name)
        })
    }
}

#[atomic_delegates]
impl ErrorMessageOmitsSource {
    pub(crate) fn message_prints_field(message: &syn::Attribute, field: &str) -> bool {}
}

// no test_usage necessary
