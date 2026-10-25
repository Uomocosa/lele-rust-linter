pub struct Sprite;
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
        app.add_systems(PostStartup, lele_bevy_preview::scene::bevy_systems::center_roots);
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
