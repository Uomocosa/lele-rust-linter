use atomic_delegate_macros::atomic_delegate;
use atomic_delegate_macros::atomic_delegates;
use lele_lint::Category;
use lele_lint::ExampleFile;
use lele_lint::Project;
use lele_lint::Rule;
use lele_lint::RuleDoc;
use lele_lint::RuleViolation;
use rule_examples_macro::rule_examples;

use crate::scan;

pub struct PreviewRouting;

impl PreviewRouting {
    pub const NAME: &'static str = "preview_routing";
    pub const CODE: &'static str = "LELEBEVY007";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Tests,
        summary: "Every `*_ui_png_preview`, `*_ui_mp4_preview` and `*_ui_scene_preview` fn must be a `#[test]` that calls `lele_bevy_preview::run(...)` and checks its result with `.expect(..)`, `.unwrap()` or `?`.",
        why: "A preview test that renders and asserts nothing can pass while producing a blank frame; routing through the harness and checking its `Result` makes `NoVisual`/`StaticClip` fail the test.",
        bad: rule_examples!("preview_routing/bad"),
        good: rule_examples!("preview_routing/good"),
    };
}

#[rustfmt::skip]
impl Rule for PreviewRouting {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(PreviewRouting)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl PreviewRouting {
    pub fn register(rules: &mut Vec<Box<dyn Rule>>) {
        rules.push(Box::new(Self));
    }
}

#[atomic_delegates]
impl PreviewRouting {
    pub(crate) fn problems(preview: &scan::Preview) -> Vec<String> {}
}

// no test_usage necessary
