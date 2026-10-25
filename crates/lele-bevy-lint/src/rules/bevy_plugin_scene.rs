use atomic_delegate_macros::atomic_delegate;
use lele_lint::Category;
use lele_lint::ExampleFile;
use lele_lint::Project;
use lele_lint::Rule;
use lele_lint::RuleDoc;
use lele_lint::RuleViolation;
use rule_examples_macro::rule_examples;

pub struct BevyPluginScene;

impl BevyPluginScene {
    pub const NAME: &'static str = "bevy_plugin_scene";
    pub const CODE: &'static str = "LELEBEVY006";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Tests,
        summary: "A file defining a `Plugin` whose build spawns UI (in its own folder, or through a system it registers directly or via its `methods/<plugin>/` delegate) must ship a `*_ui_scene_preview` test whose `Scene::build` adds that plugin.",
        why: "A plugin is the assembly point for a whole screen; its scene preview proves the assembled screen renders.",
        bad: rule_examples!("bevy_plugin_scene/bad"),
        good: rule_examples!("bevy_plugin_scene/good"),
    };
}

#[rustfmt::skip]
impl Rule for BevyPluginScene {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(BevyPluginScene)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl BevyPluginScene {
    pub fn register(rules: &mut Vec<Box<dyn Rule>>) {
        rules.push(Box::new(Self));
    }
}

// no test_usage necessary
