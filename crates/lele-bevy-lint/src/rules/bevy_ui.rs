use atomic_delegate_macros::atomic_delegate;
use lele_lint::Category;
use lele_lint::ExampleFile;
use lele_lint::Project;
use lele_lint::Rule;
use lele_lint::RuleDoc;
use lele_lint::RuleViolation;
use rule_examples_macro::rule_examples;

pub struct BevyUi;

impl BevyUi {
    pub const NAME: &'static str = "bevy_ui";
    pub const CODE: &'static str = "LELEBEVY003";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Tests,
        summary: "A file whose visual spawn is reachable from production must ship a `*_ui_png_preview` test whose `Scene::build` renders that spawn (or the file's `Plugin`) through `lele_bevy_preview::run(...)`.",
        why: "UI that is never rendered cannot be reviewed; the named preview test is the contract that forces an artifact for every production visual, and naming the real spawner keeps it from rendering a copy.",
        fix: "Add a `*_ui_png_preview` `#[test]` to the file whose `Scene::build` spawns that UI (or adds its `Plugin`) and runs it through `lele_bevy_preview::run(...)`; see the bevy-ui-preview skill.",
        bad: rule_examples!("bevy_ui/bad"),
        good: rule_examples!("bevy_ui/good"),
    };
}

#[rustfmt::skip]
impl Rule for BevyUi {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(BevyUi)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl BevyUi {
    pub fn register(rules: &mut Vec<Box<dyn Rule>>) {
        rules.push(Box::new(Self));
    }
}

// no test_usage necessary
