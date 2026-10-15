use atomic_delegate_macros::atomic_delegate;
use lele_lint::Category;
use lele_lint::Checker;
use lele_lint::ExampleFile;
use lele_lint::Project;
use lele_lint::RuleDoc;
use lele_lint::RuleViolation;

pub struct BevyUi;

impl BevyUi {
    pub const NAME: &'static str = "bevy_ui";
    pub const CODE: &'static str = "LELEBEVY003";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Tests,
        summary: "A file whose visual spawn is reachable from production must ship a `*_ui_png_preview` test whose `Scene::build` renders that spawn (or the file's `Plugin`) through `lele_bevy_preview::run(...)`.",
        why: "UI that is never rendered cannot be reviewed; the named preview test is the contract that forces an artifact for every production visual, and naming the real spawner keeps it from rendering a copy.",
        bad: &[ExampleFile {
            path: "src/discovery/ui/spawn_root.rs",
            source: r"pub struct Sprite;
pub struct Spawner;

pub fn setup(spawner: &mut Spawner) {
    spawner.spawn((Sprite,));
}
",
        }],
        good: &[ExampleFile {
            path: "src/discovery/ui/spawn_root.rs",
            source: r#"pub struct Sprite;
pub struct Spawner;

pub fn setup(spawner: &mut Spawner) {
    spawner.spawn((Sprite,));
}

#[cfg(test)]
mod tests {
    use lele_bevy_preview::run;
    use lele_bevy_preview::scene::{Kind, Scene, State};

    fn build(app: &mut App) {
        app.add_systems(Startup, super::setup);
    }

    fn rest(_world: &mut World) {}

    #[test]
    fn spawn_root_ui_png_preview() {
        let scene = Scene {
            name: String::from("spawn_root"),
            kind: Kind::System,
            build,
            states: vec![State { label: String::from("rest"), apply: rest }],
            timeline: None,
        };
        run(&scene, &Config::default(), "x").expect("preview");
    }
}
"#,
        }],
    };
}

#[rustfmt::skip]
impl Checker for BevyUi {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(BevyUi)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl BevyUi {
    pub fn register(checkers: &mut Vec<Box<dyn Checker>>) {
        checkers.push(Box::new(Self));
    }
}

// no test_usage necessary
