use crate::Category;
use crate::ExampleFile;
use crate::Project;
use crate::Rule;
use crate::RuleDoc;
use crate::RuleViolation;
use atomic_delegate_macros::atomic_delegate;
use rule_examples_macro::rule_examples;

pub struct NoPositional;

impl NoPositional {
    pub const NAME: &'static str = "no_positional";
    pub const CODE: &'static str = "LELE010";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Types,
        summary: "No positional field access (`.0`, `.1`).",
        why: "A single-field newtype derefs to its value and multi-field types have names, so a number never stands for a meaning.",
        fix: "Replace `.0` with deref (`*value`) on single-field newtypes; give tuple structs with two or more fields named fields.",
        bad: rule_examples!("no_positional/bad"),
        good: rule_examples!("no_positional/good"),
    };
}

#[rustfmt::skip]
impl Rule for NoPositional {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(NoPositional)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl NoPositional {
    pub fn register(rules: &mut Vec<Box<dyn Rule>>) {
        rules.push(Box::new(NoPositional));
    }
}

// no test_usage necessary
