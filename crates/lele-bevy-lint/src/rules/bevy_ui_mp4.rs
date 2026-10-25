use atomic_delegate_macros::atomic_delegate;
use lele_lint::Category;
use lele_lint::ExampleFile;
use lele_lint::Project;
use lele_lint::Rule;
use lele_lint::RuleDoc;
use lele_lint::RuleViolation;
use rule_examples_macro::rule_examples;

pub struct BevyUiMp4;

impl BevyUiMp4 {
    pub const NAME: &'static str = "bevy_ui_mp4";
    pub const CODE: &'static str = "LELEBEVY004";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Tests,
        summary: "A file that spawns UI reachable from production and drives it over time (`Time`, `Timer`, animation) must ship a `*_ui_mp4_preview` test whose `Scene` has a `Timeline`.",
        why: "Static frames cannot show motion; a time-driven UI needs a recorded clip to be reviewable. One-tick input changes are covered by `bevy_ui_restyle`'s before/after states instead.",
        bad: rule_examples!("bevy_ui_mp4/bad"),
        good: rule_examples!("bevy_ui_mp4/good"),
    };
}

#[rustfmt::skip]
impl Rule for BevyUiMp4 {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(BevyUiMp4)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl BevyUiMp4 {
    pub fn register(rules: &mut Vec<Box<dyn Rule>>) {
        rules.push(Box::new(Self));
    }
}

// no test_usage necessary
