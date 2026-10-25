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
    pub(crate) fn root_module_stems(project: &Project) -> HashSet<String> {
        project
            .parsed_files
            .keys()
            .filter(|rel| rel.components().count() == 1)
            .filter_map(|rel| Self::stem_name(rel))
            .collect()
    }
    pub(crate) fn stem_name(rel_path: &Path) -> Option<String> {
        rel_path
            .file_stem()
            .and_then(|s| s.to_str())
            .map(str::to_string)
    }
}

#[atomic_delegates]
impl NoStutteredPath {
    pub(crate) fn collect_stutter(
        path: &syn::Path,
        root_stems: &HashSet<String>,
        hits: &mut Vec<rules::no_stuttered_path::StutterHit>,
    ) {
    }
}

// no test_usage necessary
