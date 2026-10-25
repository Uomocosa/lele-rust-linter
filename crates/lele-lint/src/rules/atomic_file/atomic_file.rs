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
use rule_examples_macro::rule_examples;
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
        fix: "Give each extra public item its own file named after it (`pub struct Guest` → `guest.rs`), or rename the file to match its single item; then update the `mod`/`pub use` lines.",
        bad: rule_examples!("atomic_file/bad"),
        good: rule_examples!("atomic_file/good"),
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
