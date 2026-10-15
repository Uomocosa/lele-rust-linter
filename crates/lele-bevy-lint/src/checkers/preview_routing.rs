use atomic_delegate_macros::atomic_delegate;
use lele_lint::Category;
use lele_lint::Checker;
use lele_lint::ExampleFile;
use lele_lint::Project;
use lele_lint::RuleDoc;
use lele_lint::RuleViolation;

pub struct PreviewRouting;

impl PreviewRouting {
    pub const NAME: &'static str = "preview_routing";
    pub const CODE: &'static str = "LELEBEVY039";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Tests,
        summary: "Every `*_ui_png_preview`, `*_ui_mp4_preview` and `*_ui_scene_preview` fn must be a `#[test]` that calls `lele_bevy_preview::run(...)` and checks its result with `.expect(..)`, `.unwrap()` or `?`.",
        why: "A preview test that renders and asserts nothing can pass while producing a blank frame; routing through the harness and checking its `Result` makes `NoVisual`/`StaticClip` fail the test.",
        bad: &[
            ExampleFile {
                path: "src/discovery/ui/spawn_root.rs",
                source: r#"pub fn setup(s: &mut S) { s.spawn((Node,)); }

#[cfg(test)]
mod tests {
    #[test]
    fn spawn_root_ui_png_preview() {
        let shot = std::path::PathBuf::from("spawn_root.png");
        assert!(shot.exists());
    }
}
"#,
            },
            ExampleFile {
                path: "src/discovery/ui/room_row.rs",
                source: r#"pub fn setup(s: &mut S) { s.spawn((Node,)); }

#[cfg(test)]
mod tests {
    use lele_bevy_preview::run;

    #[test]
    fn room_row_ui_png_preview() {
        let _ = run(&scene(), &Config::default(), "x");
    }
}
"#,
            },
        ],
        good: &[ExampleFile {
            path: "src/discovery/ui/spawn_root.rs",
            source: r#"pub fn setup(s: &mut S) { s.spawn((Node,)); }

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
impl Checker for PreviewRouting {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(PreviewRouting)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl PreviewRouting {
    pub fn register(checkers: &mut Vec<Box<dyn Checker>>) {
        checkers.push(Box::new(Self));
    }
}

// no test_usage necessary
