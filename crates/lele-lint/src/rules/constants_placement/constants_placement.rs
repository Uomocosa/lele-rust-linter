use crate::Category;
use crate::ExampleFile;
use crate::Project;
use crate::Rule;
use crate::RuleDoc;
use crate::RuleViolation;
use atomic_delegate_macros::atomic_delegate;
use atomic_delegate_macros::atomic_delegates;
use rule_examples_macro::rule_examples;
use std::collections::HashMap;
use std::collections::HashSet;
use std::path::Path;
use std::path::PathBuf;

pub struct ConstantsPlacement;

impl ConstantsPlacement {
    pub const NAME: &'static str = "constants_placement";
    pub const CODE: &'static str = "LELE019";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Layout,
        summary: "A constant lives in the nearest `constants.rs` (or `__basic__/constants.rs`) shared by all its users.",
        why: "Constants placed too high leak across domains; placed too low they get duplicated.",
        fix: "Move the constant into `constants.rs` (or `__basic__/constants.rs`) of the nearest folder that contains every file using it, creating the file if needed, and import it from there.",
        bad: rule_examples!("constants_placement/bad"),
        good: rule_examples!("constants_placement/good"),
    };
}

#[rustfmt::skip]
impl Rule for ConstantsPlacement {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(ConstantsPlacement)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl ConstantsPlacement {
    pub fn register(rules: &mut Vec<Box<dyn Rule>>) {
        rules.push(Box::new(ConstantsPlacement));
    }
    pub(crate) fn sibling_constants(rel_path: &Path, project: &Project) -> PathBuf {
        Self::constants_target(project, &Self::parent_dir(rel_path))
    }
    pub(crate) fn parent_dir(rel_path: &Path) -> PathBuf {
        rel_path.parent().map(Path::to_path_buf).unwrap_or_default()
    }
}

#[atomic_delegates]
impl ConstantsPlacement {
    pub(crate) fn collect_defined_consts(
        parsed_files: &HashMap<PathBuf, syn::File>,
    ) -> HashMap<String, PathBuf> {
    }
    pub(crate) fn collect_const_usages(
        parsed_files: &HashMap<PathBuf, syn::File>,
        defined: &HashSet<String>,
    ) -> HashMap<String, HashSet<PathBuf>> {
    }
    pub(crate) fn check_fn_file_const(
        name: &str,
        rel_path: &Path,
        project: &Project,
    ) -> Vec<RuleViolation> {
    }
    pub(crate) fn check_lift_to_ancestor(
        name: &str,
        rel_path: &Path,
        users: &HashSet<PathBuf>,
        project: &Project,
    ) -> Vec<RuleViolation> {
    }
    pub(crate) fn check_lower_to_subdir(
        name: &str,
        rel_path: &Path,
        users: &HashSet<PathBuf>,
        project: &Project,
    ) -> Vec<RuleViolation> {
    }
    pub(crate) fn constants_target(project: &Project, dir: &Path) -> PathBuf {}
    pub(crate) fn common_ancestor_dir(users: &HashSet<PathBuf>) -> PathBuf {}
    pub(crate) fn common_prefix(a: &Path, b: &Path) -> PathBuf {}
}

// no test_usage necessary
