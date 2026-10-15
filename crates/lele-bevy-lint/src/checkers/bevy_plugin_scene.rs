use atomic_delegate_macros::atomic_delegate;
use lele_lint::Category;
use lele_lint::Checker;
use lele_lint::ExampleFile;
use lele_lint::Project;
use lele_lint::RuleDoc;
use lele_lint::RuleViolation;

pub struct BevyPluginScene;

impl BevyPluginScene {
    pub const NAME: &'static str = "bevy_plugin_scene";
    pub const CODE: &'static str = "LELEBEVY006";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Tests,
        summary: "A file defining a `Plugin` whose build spawns UI (in its own folder, or through a system it registers directly or via its `methods/<plugin>/` delegate) must ship a `*_ui_scene_preview` test whose `Scene::build` adds that plugin.",
        why: "A plugin is the assembly point for a whole screen; its scene preview proves the assembled screen renders.",
        bad: &[ExampleFile {
            path: "src/discovery/ui/default_ui_plugin.rs",
            source: r"pub fn spawn_ui(s: &mut S) { s.spawn((Node,)); }

pub struct DefaultUiPlugin;

impl Plugin for DefaultUiPlugin {
    fn build(&self, app: &mut A) { spawn_ui(app); }
}
",
        }],
        good: &[ExampleFile {
            path: "src/discovery/ui/default_ui_plugin.rs",
            source: r#"pub fn spawn_ui(s: &mut S) { s.spawn((Node,)); }

pub struct DefaultUiPlugin;

impl Plugin for DefaultUiPlugin {
    fn build(&self, app: &mut A) { spawn_ui(app); }
}

#[cfg(test)]
mod tests {
    use lele_bevy_preview::run;
    use lele_bevy_preview::scene::{Kind, Scene, State};

    fn build(app: &mut App) {
        app.add_plugins(super::DefaultUiPlugin);
    }

    fn rest(_world: &mut World) {}

    fn scene() -> Scene {
        Scene {
            name: String::from("default_ui_plugin"),
            kind: Kind::Plugin,
            build,
            states: vec![State { label: String::from("rest"), apply: rest }],
            timeline: None,
        }
    }

    #[test]
    fn default_ui_plugin_ui_png_preview() {
        run(&scene(), &Config::default(), "x").expect("preview");
    }

    #[test]
    fn default_ui_plugin_ui_scene_preview() {
        run(&scene(), &Config::default(), "x").expect("preview");
    }
}
"#,
        }],
    };
}

#[rustfmt::skip]
impl Checker for BevyPluginScene {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(BevyPluginScene)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl BevyPluginScene {
    pub fn register(checkers: &mut Vec<Box<dyn Checker>>) {
        checkers.push(Box::new(Self));
    }
}

// no test_usage necessary
