use crate::Category;
use crate::Entry;
use crate::ExampleFile;
use crate::ModuleInfoMap;
use crate::Project;
use crate::Rule;
use crate::RuleDoc;
use crate::RuleViolation;
use atomic_delegate_macros::atomic_delegate;
use atomic_delegate_macros::atomic_delegates;
use rule_examples_macro::rule_examples;
use std::collections::HashSet;

pub struct MethodVisibility;

impl MethodVisibility {
    pub const NAME: &'static str = "method_visibility";
    pub const CODE: &'static str = "LELE004";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Delegates,
        summary: "A method file is a private module: declared with `mod`, never `pub mod` or `pub use`.",
        why: "Method bodies are reached only through the type's methods, so callers can't depend on the file layout.",
        fix: "Change `pub mod <method_file>;` to `mod <method_file>;` and remove any `pub use` of it; callers use the type's method instead.",
        bad: rule_examples!("method_visibility/bad"),
        good: rule_examples!("method_visibility/good"),
    };
}

#[rustfmt::skip]
impl Rule for MethodVisibility {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(MethodVisibility)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl MethodVisibility {
    pub fn register(rules: &mut Vec<Box<dyn Rule>>) {
        rules.push(Box::new(MethodVisibility));
    }
}

#[atomic_delegates]
impl MethodVisibility {
    pub(crate) fn is_actually_method_file(
        file_name: &str,
        parent_dir: &str,
        project: &Project,
    ) -> bool {
    }
    pub(crate) fn group_entries_by_parent_dir(
        entries: &[Entry],
    ) -> std::collections::BTreeMap<String, Vec<String>> {
    }
    pub(crate) fn collect_struct_names(
        file_names: &[String],
        parent_dir: &str,
        project: &Project,
    ) -> HashSet<String> {
    }
    pub(crate) fn declared_as_pub_mod(
        module_info: &ModuleInfoMap,
        parent_dir: &str,
        mod_name: &str,
    ) -> Option<std::path::PathBuf> {
    }
    pub(crate) fn reexported_in_pub_use(
        module_info: &ModuleInfoMap,
        parent_dir: &str,
        mod_name: &str,
    ) -> Option<std::path::PathBuf> {
    }
}

// no test_usage necessary
