use crate::Category;
use crate::Checker;
use crate::ExampleFile;
use crate::Project;
use crate::RuleDoc;
use crate::RuleViolation;
use atomic_delegate_macros::atomic_delegate;

pub struct TestIsNotHelper;

impl TestIsNotHelper {
    pub const NAME: &'static str = "test_is_not_helper";
    pub const CODE: &'static str = "LELE041";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Tests,
        summary: "A `#[test]` fn is not a helper — it carries no `// needed helper:` line.",
        why: "Tests are entry points, not shared fixtures; annotating them hides the real helpers.",
        bad: &[ExampleFile {
            path: "src/timeline/tests.rs",
            source: r"#[test]
// needed helper: fixture shared by the tests in this file
fn test_usage() {}

// no test_usage necessary
",
        }],
        good: &[ExampleFile {
            path: "src/timeline/tests.rs",
            source: r"#[test]
fn test_usage() {}

// no test_usage necessary
",
        }],
    };
}

#[rustfmt::skip]
impl Checker for TestIsNotHelper {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(TestIsNotHelper)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl TestIsNotHelper {
    pub fn register(checkers: &mut Vec<Box<dyn Checker>>) {
        checkers.push(Box::new(TestIsNotHelper));
    }
}

// no test_usage necessary
