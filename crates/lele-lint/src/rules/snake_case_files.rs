use crate::Category;
use crate::ExampleFile;
use crate::Project;
use crate::Rule;
use crate::RuleDoc;
use crate::RuleViolation;
use atomic_delegate_macros::atomic_delegate;
use atomic_delegate_macros::atomic_delegates;
use rule_examples_macro::rule_examples;

pub struct SnakeCaseFiles;

impl SnakeCaseFiles {
    pub const NAME: &'static str = "snake_case_files";
    pub const CODE: &'static str = "LELE002";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Layout,
        summary: "File and folder names are snake_case.",
        why: "Module names are derived from file names; one casing keeps paths predictable.",
        bad: rule_examples!("snake_case_files/bad"),
        good: rule_examples!("snake_case_files/good"),
    };
}

#[rustfmt::skip]
impl Rule for SnakeCaseFiles {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(SnakeCaseFiles)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl SnakeCaseFiles {
    pub fn register(rules: &mut Vec<Box<dyn Rule>>) {
        rules.push(Box::new(SnakeCaseFiles));
    }
    pub(crate) fn extract_name(path: &std::path::Path, is_dir: bool) -> Option<&str> {
        if is_dir {
            path.file_name().and_then(|n| n.to_str())
        } else {
            path.file_stem().and_then(|n| n.to_str())
        }
    }
    pub(crate) fn is_allowed_dunder(project: &Project, name: &str) -> bool {
        project
            .dunder_paths
            .iter()
            .any(|dunder| dunder.file_stem() == Some(std::ffi::OsStr::new(name)))
    }
}

#[atomic_delegates]
impl SnakeCaseFiles {
    pub(crate) fn is_snake_case(name: &str) -> bool {}
}

// no test_usage necessary
