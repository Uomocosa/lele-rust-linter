use atomic_delegate_macros::atomic_delegate;
use lele_lint::Category;
use lele_lint::ExampleFile;
use lele_lint::Project;
use lele_lint::Rule;
use lele_lint::RuleDoc;
use lele_lint::RuleViolation;
use rule_examples_macro::rule_examples;

pub struct PreviewCentered;

impl PreviewCentered {
    pub const NAME: &'static str = "preview_centered";
    pub const CODE: &'static str = "LELEBEVY008";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Tests,
        summary: "A preview whose scene is `Kind::System` or `Kind::Component` must centre its UI: its `build` (or a sibling fn it reaches) must call `lele_bevy_preview::scene::centered_root()` or add `lele_bevy_preview::scene::bevy_systems::center_roots`.",
        why: "Parentless UI nodes lay out at the window's (0,0), so isolated system/component previews pile into the top-left corner, overlap each other and hide state changes; `Plugin`/`App` previews keep their real layout and are exempt.",
        bad: rule_examples!("preview_centered/bad"),
        good: rule_examples!("preview_centered/good"),
    };
}

#[rustfmt::skip]
impl Rule for PreviewCentered {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(PreviewCentered)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl PreviewCentered {
    pub fn register(rules: &mut Vec<Box<dyn Rule>>) {
        rules.push(Box::new(Self));
    }
}

// no test_usage necessary
