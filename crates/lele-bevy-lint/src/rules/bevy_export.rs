use atomic_delegate_macros::atomic_delegate;
use lele_lint::Category;
use lele_lint::ExampleFile;
use lele_lint::Project;
use lele_lint::Rule;
use lele_lint::RuleDoc;
use lele_lint::RuleViolation;
use rule_examples_macro::rule_examples;

pub struct BevyExport;

impl BevyExport {
    pub const NAME: &'static str = "bevy_export";
    pub const CODE: &'static str = "LELEBEVY001";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Layout,
        summary: "A domain declares `pub mod bevy_systems;` but never re-exports its systems at the domain root.",
        why: "Consumers reach systems as `domain::bevy_systems::system`, so the folder is always visible in the path.",
        bad: rule_examples!("bevy_export/bad"),
        good: rule_examples!("bevy_export/good"),
    };
}

#[rustfmt::skip]
impl Rule for BevyExport {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(BevyExport)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl BevyExport {
    pub fn register(rules: &mut Vec<Box<dyn Rule>>) {
        rules.push(Box::new(Self));
    }
}

// no test_usage necessary
