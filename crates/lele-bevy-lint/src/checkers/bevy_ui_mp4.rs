use atomic_delegate_macros::atomic_delegate;
use lele_lint::Category;
use lele_lint::Checker;
use lele_lint::ExampleFile;
use lele_lint::Project;
use lele_lint::RuleDoc;
use lele_lint::RuleViolation;

pub struct BevyUiMp4;

impl BevyUiMp4 {
    pub const NAME: &'static str = "bevy_ui_mp4";
    pub const CODE: &'static str = "LELEBEVY004";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Tests,
        summary: "A file that spawns UI reachable from production and drives it over time (`Time`, `Timer`, animation) must ship a `*_ui_mp4_preview` test whose `Scene` has a `Timeline`.",
        why: "Static frames cannot show motion; a time-driven UI needs a recorded clip to be reviewable. One-tick input changes are covered by `bevy_ui_restyle`'s before/after states instead.",
        bad: &[ExampleFile {
            path: "src/discovery/ui/sync_room_list.rs",
            source: r"pub fn setup(s: &mut S) { s.spawn((Node,)); }

pub fn tick(time: Res<Time>) {
    let _ = time.delta_secs();
}
",
        }],
        good: &[ExampleFile {
            path: "src/discovery/ui/sync_room_list.rs",
            source: r#"pub fn setup(s: &mut S) { s.spawn((Node,)); }

pub fn tick(time: Res<Time>) { let _ = time.delta_secs(); }

#[cfg(test)]
mod tests {
    use lele_bevy_preview::run;
    use lele_bevy_preview::scene::{Kind, Scene, State, Timeline};

    fn build(app: &mut App) {
        app.add_systems(Startup, super::setup).add_systems(Update, super::tick);
    }

    fn rest(_world: &mut World) {}

    fn scene(timeline: Option<Timeline>) -> Scene {
        Scene {
            name: String::from("sync_room_list"),
            kind: Kind::System,
            build,
            states: vec![State { label: String::from("rest"), apply: rest }],
            timeline,
        }
    }

    #[test]
    fn sync_room_list_ui_png_preview() {
        run(&scene(None), &Config::default(), "x").expect("preview");
    }

    #[test]
    fn sync_room_list_ui_mp4_preview() {
        let clip = Timeline { label: String::from("tick"), frames: 30, fps: 30, apply: rest };
        run(&scene(Some(clip)), &Config::default(), "x").expect("preview");
    }
}
"#,
        }],
    };
}

#[rustfmt::skip]
impl Checker for BevyUiMp4 {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(BevyUiMp4)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl BevyUiMp4 {
    pub fn register(checkers: &mut Vec<Box<dyn Checker>>) {
        checkers.push(Box::new(Self));
    }
}

// no test_usage necessary
