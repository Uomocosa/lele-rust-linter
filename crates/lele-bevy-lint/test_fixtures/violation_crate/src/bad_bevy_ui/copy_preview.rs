// The preview renders a hand-copied twin of the production spawner, so edits
// to `spawn_status` never change the artifact (LELEBEVY029).

pub struct Spawner;

pub fn setup(spawner: &mut Spawner) {
    spawn_status(spawner);
}

pub fn spawn_status(spawner: &mut Spawner) {
    spawner.spawn((Text,));
}

#[cfg(test)]
mod tests {
    use lele_bevy_preview::run;
    use lele_bevy_preview::scene::{Kind, Scene, State};

    fn copy(spawner: &mut Spawner) {
        spawner.spawn((Text,));
    }

    fn build(app: &mut App) {
        let _ = super::spawn_status;
        app.add_systems(Startup, copy);
    }

    fn rest(_world: &mut World) {}

    #[test]
    fn status_ui_png_preview() {
        let scene = Scene {
            name: String::from("status"),
            kind: Kind::App,
            build,
            states: vec![State {
                label: String::from("rest"),
                apply: rest,
            }],
            timeline: None,
        };
        run(&scene, &Config::default(), "violation_crate").expect("preview");
    }
}
