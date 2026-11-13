use crate::common;
use crate::traits::Rule;
use crate::Category;
use crate::ExampleFile;
use crate::Project;
use crate::RuleDoc;
use crate::RuleViolation;
use atomic_delegate_macros::atomic_delegate;
use atomic_delegate_macros::atomic_delegates;
use rule_examples_macro::rule_examples;
use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::path::Path;

pub struct MethodsLayout;

impl MethodsLayout {
    pub const NAME: &'static str = "methods_layout";
    pub const CODE: &'static str = "LELE030";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Delegates,
        summary: "Every `#[atomic_delegates]` shell has a body at `__methods__/<type>/<method>.rs` with `pub fn <method>` and its own `test_usage` (or a final `// no test_usage necessary`); a body that is a one-liner is not a delegate and is reported: it is inlined in the type file. A leftover crate-root `methods/` body folder is reported for renaming.",
        why: "The `__methods__/` folder (mounted as the `crate::methods` module) hides bodies until you need them; the type file stays a short list of what the type can do. A single statement spanning several lines is a real body and may stay in its delegate file.",
        fix: "Create `__methods__/<type>/<method>.rs` with `pub fn <method>` (the receiver becomes the first argument) and a `test_usage` test, then run `lele-lint --sync-methods <crate>` to update the `__methods__/` `mod.rs` indexes. If the body is a single one-line statement, do not create the file: write the method inline in a `#[rustfmt::skip] impl` block, then delete the file. Never pad a body or change a signature to keep a delegate file. To rename a stale root-level `methods/` folder: `git mv methods __methods__` and update the `#[path]` mount to `#[path = \"../__methods__/mod.rs\"]`.",
        bad: rule_examples!("methods_layout/bad"),
        good: rule_examples!("methods_layout/good"),
    };
}

#[rustfmt::skip]
impl Rule for MethodsLayout {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(MethodsLayout)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl MethodsLayout {
    pub fn register(rules: &mut Vec<Box<dyn Rule>>) {
        rules.push(Box::new(MethodsLayout));
    }
    pub(crate) fn is_index_file(rel: &Path) -> bool {
        Self::file_stem(rel).is_some_and(|stem| stem == "mod")
    }
}

#[atomic_delegates]
impl MethodsLayout {
    pub(crate) fn defines_pub_fn(file: &syn::File, name: &str) -> bool {}
    pub(crate) fn pub_fn_block<'a>(file: &'a syn::File, name: &str) -> Option<&'a syn::Block> {}
    pub(crate) fn parent_dir(rel: &Path) -> Option<String> {}
    pub(crate) fn file_stem(rel: &Path) -> Option<String> {}
    pub(crate) fn file_violation(project: &Project, rel: &Path, message: String) -> RuleViolation {}
    pub(crate) fn read_methods_file(project: &Project, rel: &Path) -> Option<String> {}
    pub(crate) fn check_body(
        project: &Project,
        type_snake: &str,
        method: &str,
        rel: &Path,
        file: &syn::File,
    ) -> Vec<RuleViolation> {
    }
    pub(crate) fn check_type_index(
        project: &Project,
        type_snake: &str,
        declared_type: &common::DeclaredType,
    ) -> Vec<RuleViolation> {
    }
    pub(crate) fn check_type_indexes(
        project: &Project,
        shell_roots: &BTreeMap<(String, String), BTreeSet<String>>,
        type_snake: &str,
    ) -> Vec<RuleViolation> {
    }
    pub(crate) fn check_orphans(
        project: &Project,
        declared: &BTreeMap<String, common::DeclaredType>,
        shell_roots: &BTreeMap<(String, String), BTreeSet<String>>,
        multi: bool,
    ) -> Vec<RuleViolation> {
    }
    pub(crate) fn wrong_root_message(
        project: &Project,
        rel: &Path,
        actual_root: Option<&String>,
        type_snake: &str,
        expected: Option<&BTreeSet<String>>,
    ) -> String {
    }
    pub(crate) fn check_root_index(
        project: &Project,
        declared: &BTreeMap<String, common::DeclaredType>,
    ) -> Vec<RuleViolation> {
    }
    pub(crate) fn check_root_indexes(
        project: &Project,
        declared: &BTreeMap<String, common::DeclaredType>,
    ) -> Vec<RuleViolation> {
    }
    pub(crate) fn check_duplicates(project: &Project) -> Vec<RuleViolation> {}
    pub(crate) fn check_root_collisions(project: &Project) -> Vec<RuleViolation> {}
    pub(crate) fn check_stale_methods_folder(project: &Project) -> Vec<RuleViolation> {}
}

// no test_usage necessary
