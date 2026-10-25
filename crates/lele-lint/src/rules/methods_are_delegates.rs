use crate::Category;
use crate::ExampleFile;
use crate::Project;
use crate::Rule;
use crate::RuleDoc;
use crate::RuleViolation;
use atomic_delegate_macros::atomic_delegate;
use rule_examples_macro::rule_examples;

pub struct MethodsAreDelegates;

impl MethodsAreDelegates {
    pub const NAME: &'static str = "methods_are_delegates";
    pub const CODE: &'static str = "LELE035";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Delegates,
        summary: "A `methods/<type>/<method>.rs` file holds only `use` lines, private `const`s, its delegate body `pub fn <method>` and `#[cfg(test)] mod tests` — no helper fns, types or impls.",
        why: "`methods/` holds bodies of the functions `src/` declares and nothing else; a helper or type there is machinery nobody finds from the type, so it becomes a `pub(crate)` method or type in `src/`.",
        bad: rule_examples!("methods_are_delegates/bad"),
        good: rule_examples!("methods_are_delegates/good"),
    };
}

#[rustfmt::skip]
impl Rule for MethodsAreDelegates {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(MethodsAreDelegates)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl MethodsAreDelegates {
    pub fn register(rules: &mut Vec<Box<dyn Rule>>) {
        rules.push(Box::new(MethodsAreDelegates));
    }
}

// no test_usage necessary
