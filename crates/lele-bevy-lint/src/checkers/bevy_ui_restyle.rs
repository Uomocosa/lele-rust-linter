use atomic_delegate_macros::atomic_delegate;
use lele_lint::Category;
use lele_lint::Checker;
use lele_lint::ExampleFile;
use lele_lint::Project;
use lele_lint::RuleDoc;
use lele_lint::RuleViolation;

pub struct BevyUiRestyle;

impl BevyUiRestyle {
    pub const NAME: &'static str = "bevy_ui_restyle";
    pub const CODE: &'static str = "LELEBEVY005";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Tests,
        summary: "A file with a system that restyles UI (`&mut` on a styling component) must ship a preview that renders that system: a `*_ui_png_preview` with at least two `State`s when it is driven by input or state, or a `*_ui_mp4_preview` when it is driven by time.",
        why: "A restyle never changes the set of entities, so a spawn-only preview cannot see it; only a rendering that names the restyle system (with a before/after pair, or a clip for motion) proves the change lands.",
        bad: &[ExampleFile {
            path: "src/discovery/ui/bevy_systems/gate_create.rs",
            source: r"pub fn build(app: &mut App) {
    app.add_systems(Update, gate_create);
}

pub fn gate_create(button: Query<&mut BackgroundColor>) {
    let _ = button;
}
",
        }],
        good: &[ExampleFile {
            path: "src/discovery/ui/bevy_systems/gate_create.rs",
            source: r#"pub fn build(app: &mut App) {
    app.add_systems(Update, gate_create);
}

pub fn gate_create(button: Query<&mut BackgroundColor>) {
    let _ = button;
}

#[cfg(test)]
mod tests {
    use lele_bevy_preview::run;
    use lele_bevy_preview::scene::{Kind, Scene, State};

    use super::gate_create;

    fn preview_build(app: &mut App) {
        app.add_systems(Update, gate_create);
        app.add_systems(PostStartup, lele_bevy_preview::scene::center_roots);
    }

    fn rest(_world: &mut World) {}

    #[test]
    fn gate_create_ui_png_preview() {
        let scene = Scene {
            name: String::from("gate_create"),
            kind: Kind::System,
            build: preview_build,
            states: vec![
                State { label: String::from("blank"), apply: rest },
                State { label: String::from("typed"), apply: rest },
            ],
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
impl Checker for BevyUiRestyle {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(BevyUiRestyle)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl BevyUiRestyle {
    pub fn register(checkers: &mut Vec<Box<dyn Checker>>) {
        checkers.push(Box::new(Self));
    }
}

// no test_usage necessary
