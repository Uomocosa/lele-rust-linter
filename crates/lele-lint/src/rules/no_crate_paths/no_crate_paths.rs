use crate::Category;
use crate::ExampleFile;
use crate::Project;
use crate::Rule;
use crate::RuleDoc;
use crate::RuleViolation;
use atomic_delegate_macros::atomic_delegate;
use rule_examples_macro::rule_examples;

pub struct NoCratePaths;

impl NoCratePaths {
    pub const NAME: &'static str = "no_crate_paths";
    pub const CODE: &'static str = "LELE025";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Imports,
        summary: "`crate::` appears only in `use` items (outside `lib.rs`/`main.rs`).",
        why: "All cross-domain dependencies of a file are then visible in its `use` lines at the top.",
        bad: rule_examples!("no_crate_paths/bad"),
        good: rule_examples!("no_crate_paths/good"),
    };
}

#[rustfmt::skip]
impl Rule for NoCratePaths {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(NoCratePaths)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl NoCratePaths {
    pub fn register(rules: &mut Vec<Box<dyn Rule>>) {
        rules.push(Box::new(NoCratePaths));
    }
}

// no test_usage necessary
