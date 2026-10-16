use atomic_delegate_macros::atomic_delegate;
use lele_lint::Category;
use lele_lint::Checker;
use lele_lint::ExampleFile;
use lele_lint::Project;
use lele_lint::RuleDoc;
use lele_lint::RuleViolation;

pub struct PreviewCentered;

impl PreviewCentered {
    pub const NAME: &'static str = "preview_centered";
    pub const CODE: &'static str = "LELEBEVY008";
    pub const DOC: RuleDoc = RuleDoc {
        category: Category::Tests,
        summary: "A preview whose scene is `Kind::System` or `Kind::Component` must centre its UI: its `build` (or a sibling fn it reaches) must call `lele_bevy_preview::scene::centered_root()` or add `lele_bevy_preview::scene::center_roots`.",
        why: "Parentless UI nodes lay out at the window's (0,0), so isolated system/component previews pile into the top-left corner, overlap each other and hide state changes; `Plugin`/`App` previews keep their real layout and are exempt.",
        bad: &[ExampleFile {
            path: "src/discovery/ui/room_row.rs",
            source: r#"pub fn highlight(s: &mut S) {}

#[cfg(test)]
mod tests {
    use lele_bevy_preview::run;
    use lele_bevy_preview::scene::{Kind, Scene, State};

    fn spawn_rows(mut commands: Commands) {
        commands.spawn((Node::default(),));
    }

    fn build(app: &mut App) {
        app.add_systems(Startup, spawn_rows);
        app.add_systems(Update, super::highlight);
    }

    fn rest(_world: &mut World) {}

    #[test]
    fn room_row_ui_png_preview() {
        let scene = Scene {
            name: String::from("room_row"),
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
        good: &[ExampleFile {
            path: "src/discovery/ui/room_row.rs",
            source: r#"pub fn highlight(s: &mut S) {}

#[cfg(test)]
mod tests {
    use lele_bevy_preview::run;
    use lele_bevy_preview::scene::{Kind, Scene, State};

    fn spawn_rows(mut commands: Commands) {
        let root = commands.spawn(lele_bevy_preview::scene::centered_root()).id();
        commands.spawn((ChildOf(root), Node::default()));
    }

    fn build(app: &mut App) {
        app.add_systems(Startup, spawn_rows);
        app.add_systems(Update, super::highlight);
    }

    fn rest(_world: &mut World) {}

    #[test]
    fn room_row_ui_png_preview() {
        let scene = Scene {
            name: String::from("room_row"),
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
impl Checker for PreviewCentered {
    fn name(&self) -> &'static str { Self::NAME }
    fn code(&self) -> &'static str { Self::CODE }
    fn doc(&self) -> RuleDoc { Self::DOC }
    #[atomic_delegate(PreviewCentered)]
    fn check(&self, project: &Project) -> Vec<RuleViolation> {}
}

#[rustfmt::skip]
impl PreviewCentered {
    pub fn register(checkers: &mut Vec<Box<dyn Checker>>) {
        checkers.push(Box::new(Self));
    }
}

// no test_usage necessary
