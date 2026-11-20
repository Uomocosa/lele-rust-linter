use crate::traits::Rule;
use crate::Category;
use crate::ExampleFile;
use crate::FlatDirEntry;
use crate::Project;
use crate::RuleDoc;
use crate::RuleViolation;
use atomic_delegate_macros::atomic_delegate;
use atomic_delegate_macros::atomic_delegates;
use rule_examples_macro::rule_examples;
use std::path::Path;
use std::path::PathBuf;

pub struct FlatDir;

impl FlatDir {
    pub const NAME: &'static str = "flat_dir";
    pub const CODE: &'static str = "LELE044";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Layout,
        summary: "A `[[lele.flat_dir]]` dir holds only files, no subdirectories.",
        why: "A flat dir holds one file per item with shared items at the crate root; a subdirectory hides files from that layout and invites the nesting the flat layout forbids.",
        fix: "Move the nested files up into the flat dir itself, or delete the `[[lele.flat_dir]]` entry if the dir is meant to nest.",
        bad: rule_examples!("flat_dir/bad"),
        good: rule_examples!("flat_dir/good"),
    };
}

#[rustfmt::skip]
impl Rule for FlatDir {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(FlatDir, Rule)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl FlatDir {
    pub fn register(rules: &mut Vec<Box<dyn Rule>>) {
        rules.push(Box::new(FlatDir));
    }
}

impl FlatDir {
    pub(crate) fn crate_relative_path(project: &Project, rel_path: &Path) -> PathBuf {
        if project.src_index_for_rel(rel_path) == 0 {
            Path::new("src").join(rel_path)
        } else {
            rel_path.to_path_buf()
        }
    }
}

#[atomic_delegates]
impl FlatDir {
    pub(crate) fn validate_flat_dir(flat: &FlatDirEntry, base: &Path) -> Vec<RuleViolation> {}
}

// no test_usage necessary
