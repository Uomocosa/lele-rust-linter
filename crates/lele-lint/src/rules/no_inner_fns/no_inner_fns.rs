use crate::Category;
use crate::ExampleFile;
use crate::Project;
use crate::Rule;
use crate::RuleDoc;
use crate::RuleViolation;
use atomic_delegate_macros::atomic_delegate;
use rule_examples_macro::rule_examples;

pub struct NoInnerFns;

impl NoInnerFns {
    pub const NAME: &'static str = "no_inner_fns";
    pub const CODE: &'static str = "LELE021";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Delegates,
        summary: "No `fn` inside a function body, except a tree walk's inner `fn recursion`.",
        why: "A nested helper hides from the type that owns it; only the recursion's own walk belongs inside its public function (RATIONALE §11).",
        bad: rule_examples!("no_inner_fns/bad"),
        good: rule_examples!("no_inner_fns/good"),
    };
}

#[rustfmt::skip]
impl Rule for NoInnerFns {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(NoInnerFns)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl NoInnerFns {
    pub fn register(rules: &mut Vec<Box<dyn Rule>>) {
        rules.push(Box::new(NoInnerFns));
    }
}

// no test_usage necessary
