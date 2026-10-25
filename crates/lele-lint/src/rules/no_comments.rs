use rule_examples_macro::rule_examples;
use std::path::Path;

use crate::Category;
use crate::Entry;
use crate::ExampleFile;
use crate::Project;
use crate::Rule;
use crate::RuleDoc;
use crate::RuleViolation;
use atomic_delegate_macros::atomic_delegate;
use atomic_delegate_macros::atomic_delegates;

pub struct NoComments;

impl NoComments {
    pub const NAME: &'static str = "no_comments";
    pub const CODE: &'static str = "LELE031";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Style,
        summary: "No comments in `src/` or `methods/`, except a final `// no test_usage necessary`.",
        why: "Names, signatures and the usage test carry the meaning; comments drift from the code they describe.",
        bad: rule_examples!("no_comments/bad"),
        good: rule_examples!("no_comments/good"),
    };
}

#[rustfmt::skip]
impl Rule for NoComments {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(NoComments)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl NoComments {
    pub fn register(rules: &mut Vec<Box<dyn Rule>>) {
        rules.push(Box::new(NoComments));
    }
    pub(crate) fn is_allowlisted(text: &str) -> bool { text.trim() == "// no test_usage necessary" }
}

#[atomic_delegates]
impl NoComments {
    pub(crate) fn scan_entries(entries: &[Entry], base: &Path) -> Vec<RuleViolation> {}
}

// no test_usage necessary
