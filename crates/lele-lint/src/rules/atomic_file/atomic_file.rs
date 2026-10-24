use crate::common;
use crate::rules;
use crate::Category;
use crate::ExampleFile;
use crate::Project;
use crate::Rule;
use crate::RuleDoc;
use crate::RuleViolation;
use atomic_delegate_macros::atomic_delegate;
use atomic_delegate_macros::atomic_delegates;
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::path::Path;

pub struct AtomicFile;

impl AtomicFile {
    pub const NAME: &'static str = "atomic_file";
    pub const CODE: &'static str = "LELE001";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Layout,
        summary: "One public (`pub` or `pub(crate)`) item per file, and the file is named after it (`guest.rs` holds `pub struct Guest`).",
        why: "A file name tells you exactly what is inside; finding code means finding a file, and every unit gets its own test.",
        bad: &[
            ExampleFile {
                path: "src/lib.rs",
                source: r"mod greeting;
pub use greeting::{farewell, greet};
",
            },
            ExampleFile {
                path: "src/greeting.rs",
                source: r#"pub fn greet(name: &str) -> String {
    format!("Hello, {name}!")
}

pub fn farewell(name: &str) -> String {
    format!("Bye, {name}!")
}
"#,
            },
        ],
        good: &[
            ExampleFile {
                path: "src/lib.rs",
                source: r"mod guest;
mod host;
pub use guest::Guest;
pub use host::Host;
",
            },
            ExampleFile {
                path: "src/guest.rs",
                source: r#"use derive_more::Deref;

#[derive(Deref)]
pub struct Guest(pub String);

#[rustfmt::skip]
impl Guest {
    pub fn greet(&self) -> String { format!("Hello, {}!", **self) }
}

#[cfg(test)]
mod tests {
    use super::Guest;

    #[test]
    fn test_usage() {
        assert_eq!(Guest("Ada".to_string()).greet(), "Hello, Ada!");
    }
}
"#,
            },
            ExampleFile {
                path: "src/host.rs",
                source: r#"use derive_more::Deref;

#[derive(Deref)]
pub struct Host(pub String);

#[rustfmt::skip]
impl Host {
    pub fn farewell(&self) -> String { format!("Bye, {}!", **self) }
}

#[cfg(test)]
mod tests {
    use super::Host;

    #[test]
    fn test_usage() {
        assert_eq!(Host("Ada".to_string()).farewell(), "Bye, Ada!");
    }
}
"#,
            },
        ],
    };
}

#[rustfmt::skip]
impl Rule for AtomicFile {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(AtomicFile)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl AtomicFile {
    pub fn register(rules: &mut Vec<Box<dyn Rule>>) {
        rules.push(Box::new(AtomicFile));
    }
    pub(crate) fn known_type_stems(project: &Project) -> HashSet<String> {
        project
            .parsed_files
            .iter()
            .filter_map(|(rel_path, file)| {
                let stem = rel_path.file_stem()?.to_str()?;
                common::SourceFile(file).primary_type_name(stem)?;
                Some(stem.to_string())
            })
            .collect()
    }
}

#[atomic_delegates]
impl AtomicFile {
    pub(crate) fn check_atomic_file(
        file: &syn::File,
        rel_path: &Path,
        project: &Project,
        known_stems: &HashSet<String>,
    ) -> Vec<RuleViolation> {
    }
    pub(crate) fn check_delegate_shape(
        rel_path: &Path,
        file: &syn::File,
        project: &Project,
        known_stems: &HashSet<String>,
        declared: &BTreeMap<String, common::DeclaredType>,
    ) -> Vec<RuleViolation> {
    }
    pub(crate) fn check_fn_file_purity(
        file: &syn::File,
        file_stem: &str,
        rel_path: &Path,
        project: &Project,
    ) -> Vec<RuleViolation> {
    }
    pub(crate) fn collect_pub_items(file: &syn::File) -> Vec<rules::atomic_file::PubItem> {}
    pub(crate) fn check_filename_match(
        name: &str,
        file_stem: &str,
        rel_path: &Path,
        project: &Project,
        known_stems: &HashSet<String>,
    ) -> Vec<RuleViolation> {
    }
}

// no test_usage necessary
