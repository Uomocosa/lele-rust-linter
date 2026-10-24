// The preview swallows the harness result, so a blank frame still passes, and
// a second preview fn is missing `#[test]`, so it never runs (LELEBEVY007).

pub struct Spawner;

pub fn setup(spawner: &mut Spawner) {
    spawner.spawn((Node,));
}

#[cfg(test)]
mod tests {
    use lele_bevy_preview::run;
    use lele_bevy_preview::scene::{Kind, Scene, State};

    fn build(app: &mut App) {
        app.add_systems(Startup, super::setup);
        app.add_systems(PostStartup, lele_bevy_preview::scene::bevy_systems::center_roots);
    }

    fn rest(_world: &mut World) {}

    fn scene() -> Scene {
        Scene {
            name: String::from("discarded"),
            kind: Kind::System,
            build,
            states: vec![State {
                label: String::from("rest"),
                apply: rest,
            }],
            timeline: None,
        }
    }

    #[test]
    fn discarded_ui_png_preview() {
        let _ = run(&scene(), &Config::default(), "violation_crate");
    }

    fn untested_ui_scene_preview() {
        run(&scene(), &Config::default(), "violation_crate").expect("preview");
    }
}
