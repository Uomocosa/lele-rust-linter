use crate::Category;
use crate::ExampleFile;
use crate::Project;
use crate::Rule;
use crate::RuleDoc;
use crate::RuleViolation;
use atomic_delegate_macros::atomic_delegate;

pub struct ClippyConfigClippy;

impl ClippyConfigClippy {
    pub const NAME: &'static str = "clippy_config_clippy";
    pub const CODE: &'static str = "LELE029";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Config,
        summary: "`clippy.toml` sets `allow-unwrap-in-tests`, `allow-expect-in-tests`, `allow-panic-in-tests` and `allow-indexing-slicing-in-tests` to `true`.",
        why: "Tests may unwrap and index freely; production code may not.",
        bad: &[
            ExampleFile {
                path: "clippy.toml",
                source: r"allow-unwrap-in-tests = true
",
            },
        ],
        good: &[
            ExampleFile {
                path: "clippy.toml",
                source: r"allow-unwrap-in-tests = true
allow-expect-in-tests = true
allow-panic-in-tests = true
allow-indexing-slicing-in-tests = true
",
            },
        ],
    };
}

#[rustfmt::skip]
impl Rule for ClippyConfigClippy {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(ClippyConfigClippy)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl ClippyConfigClippy {
    pub fn register(rules: &mut Vec<Box<dyn Rule>>) {
        rules.push(Box::new(ClippyConfigClippy));
    }
}

// no test_usage necessary
