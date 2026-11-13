use crate::rules;
use crate::traits::Rule;
use crate::Category;
use crate::ExampleFile;
use crate::Project;
use crate::RuleDoc;
use crate::RuleViolation;
use atomic_delegate_macros::atomic_delegate;
use atomic_delegate_macros::atomic_delegates;
use rule_examples_macro::rule_examples;
use std::collections::HashSet;
use std::path::Path;

pub struct NoStutteredPath;

impl NoStutteredPath {
    pub const NAME: &'static str = "no_stuttered_path";
    pub const CODE: &'static str = "LELE011";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Imports,
        summary: "No `meters::Meters` paths for crate-root modules: import the type once and write `Meters`.",
        why: "The module and type say the same word twice; the type name alone is already unambiguous.",
        fix: "Import the type once (`use crate::Meters;`) and write `Meters` instead of `meters::Meters`.",
        bad: rule_examples!("no_stuttered_path/bad"),
        good: rule_examples!("no_stuttered_path/good"),
    };
}

#[rustfmt::skip]
impl Rule for NoStutteredPath {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(NoStutteredPath)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl NoStutteredPath {
    pub fn register(rules: &mut Vec<Box<dyn Rule>>) {
        rules.push(Box::new(NoStutteredPath));
    }
}

impl NoStutteredPath {
    pub(crate) fn root_module_stems(project: &Project) -> HashSet<String> {
        let mut stems = HashSet::new();
        for rel in project.parsed_files.keys() {
            let count = rel.components().count();
            let is_extra_top = count == 2 && project.src_index_for_rel(rel) != 0;
            if count == 1 || is_extra_top {
                if let Some(stem) = Self::stem_name(rel) {
                    stems.insert(stem);
                }
            }
        }
        stems
    }
}

#[atomic_delegates]
impl NoStutteredPath {
    pub(crate) fn stem_name(rel_path: &Path) -> Option<String> {}
    pub(crate) fn collect_stutter(
        path: &syn::Path,
        root_stems: &HashSet<String>,
        hits: &mut Vec<rules::no_stuttered_path::StutterHit>,
    ) {
    }
}

// no test_usage necessary
