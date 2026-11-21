use crate::traits::Rule;
use crate::Category;
use crate::ExampleFile;
use crate::Project;
use crate::RuleDoc;
use crate::RuleViolation;
use crate::TestingRole;
use atomic_delegate_macros::atomic_delegate;
use atomic_delegate_macros::atomic_delegates;
use rule_examples_macro::rule_examples;
use std::path::Path;

pub struct TestingLayout;

impl TestingLayout {
    pub const NAME: &'static str = "testing_layout";
    pub const CODE: &'static str = "LELETEST006";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Tests,
        summary: "`src/testing/` holds only `mod.rs`, `__basic__/` and the four roles `__app__/`, `__queries__/`, `__fixtures__/`, `__fakes__/`.",
        why: "One place per kind of test helper means the next test finds the app builder, the world query, the fixture, the fake or the shared behaviour-free item instead of copying it (RATIONALE §17).",
        fix: "Move the file into `src/testing/__basic__/` (a behaviour-free item shared by two or more roles), `src/testing/__app__/` (an app builder), `src/testing/__queries__/` (a world query), `src/testing/__fixtures__/` (plain data) or `src/testing/__fakes__/` (a fake), or keep `src/testing/mod.rs` as the re-export.",
        bad: rule_examples!("testing_layout/bad"),
        good: rule_examples!("testing_layout/good"),
    };
}

pub(crate) const TESTING_ROLES: [&str; 4] = ["__app__", "__queries__", "__fixtures__", "__fakes__"];

pub(crate) const TESTING_ROLE_TABLE: [(&str, TestingRole); 5] = [
    ("__app__", TestingRole::App),
    ("__queries__", TestingRole::Queries),
    ("__fixtures__", TestingRole::Fixtures),
    ("__fakes__", TestingRole::Fakes),
    ("__basic__", TestingRole::Basic),
];

#[rustfmt::skip]
impl Rule for TestingLayout {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(TestingLayout, Rule)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl TestingLayout {
    pub fn register(rules: &mut Vec<Box<dyn Rule>>) {
        rules.push(Box::new(TestingLayout));
    }
}

#[atomic_delegates]
impl TestingLayout {
    pub(crate) fn is_testing_path(rel_path: &Path) -> bool {}
    pub(crate) fn offending_component(rel_path: &Path) -> Option<&str> {}
}

// no test_usage necessary
