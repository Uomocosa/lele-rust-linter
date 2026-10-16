// Spawns UI and drives it over time, but only ships a png preview (LELEBEVY004).

pub struct Spawner;

pub fn setup(spawner: &mut Spawner) {
    spawner.spawn((Node,));
}

pub fn tick(time: Res<Time>) {
    let _ = time.delta_secs();
}

#[cfg(test)]
mod tests {
    use lele_bevy_preview::run;
    use lele_bevy_preview::scene::{Kind, Scene, State};

    fn build(app: &mut App) {
        app.add_systems(Startup, super::setup);
        app.add_systems(PostStartup, lele_bevy_preview::scene::center_roots);
    }

    fn rest(_world: &mut World) {}

    #[test]
    fn missing_mp4_ui_png_preview() {
        let scene = Scene {
            name: String::from("missing_mp4"),
            kind: Kind::System,
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
