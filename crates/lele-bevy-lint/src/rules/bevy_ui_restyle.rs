use atomic_delegate_macros::atomic_delegate;
use lele_lint::Category;
use lele_lint::ExampleFile;
use lele_lint::Project;
use lele_lint::Rule;
use lele_lint::RuleDoc;
use lele_lint::RuleViolation;
use rule_examples_macro::rule_examples;

use crate::scan;

pub struct BevyUiRestyle;

impl BevyUiRestyle {
    pub const NAME: &'static str = "bevy_ui_restyle";
    pub const CODE: &'static str = "LELEBEVY005";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Tests,
        summary: "A file with a system that restyles UI (`&mut` on a styling component) must ship a preview that renders that system: a `*_ui_png_preview` with at least two `State`s when it is driven by input or state, or a `*_ui_mp4_preview` when it is driven by time.",
        why: "A restyle never changes the set of entities, so a spawn-only preview cannot see it; only a rendering that names the restyle system (with a before/after pair, or a clip for motion) proves the change lands.",
        fix: "Add a preview that runs the restyling system: a `*_ui_png_preview` with two `State`s (before, after) for input or state changes, or a `*_ui_mp4_preview` for time-driven changes.",
        bad: rule_examples!("bevy_ui_restyle/bad"),
        good: rule_examples!("bevy_ui_restyle/good"),
    };
}

#[rustfmt::skip]
impl Rule for BevyUiRestyle {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(BevyUiRestyle)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl BevyUiRestyle {
    pub fn register(rules: &mut Vec<Box<dyn Rule>>) {
        rules.push(Box::new(Self));
    }
    pub(crate) fn suffix(kind: scan::PreviewKind) -> &'static str {
        match kind {
            scan::PreviewKind::Png => "png",
            scan::PreviewKind::Mp4 | scan::PreviewKind::Scene => "mp4",
        }
    }
}

// no test_usage necessary
