use crate::Category;
use crate::ExampleFile;
use crate::Project;
use crate::Rule;
use crate::RuleDoc;
use crate::RuleViolation;
use atomic_delegate_macros::atomic_delegate;
use rule_examples_macro::rule_examples;

pub struct HelpersAreMethods;

impl HelpersAreMethods {
    pub const NAME: &'static str = "helpers_are_methods";
    pub const CODE: &'static str = "LELE008";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Delegates,
        summary: "No private `fn` in `src/`: a helper is a `pub(crate)` method or associated fn of the type it serves, never a private free fn or a private method. `fn main` in `main.rs` or a `src/bin/*.rs` binary and test code are exempt.",
        why: "`src/` lists types and what each can do; `pub` vs `pub(crate)` on the type's own methods tells API from internal machinery, so no helper hides outside the type.",
        fix: "Turn the private `fn` into a `pub(crate)` method or associated fn of the type it serves: an inline one-liner, or an `#[atomic_delegates]` shell with its body in `methods/<type>/<fn>.rs`.",
        bad: rule_examples!("helpers_are_methods/bad"),
        good: rule_examples!("helpers_are_methods/good"),
    };
}

#[rustfmt::skip]
impl Rule for HelpersAreMethods {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(HelpersAreMethods)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl HelpersAreMethods {
    pub fn register(rules: &mut Vec<Box<dyn Rule>>) {
        rules.push(Box::new(HelpersAreMethods));
    }
}

// no test_usage necessary
