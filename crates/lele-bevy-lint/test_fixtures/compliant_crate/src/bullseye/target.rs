// Spawns a UI visual and ships the required ui_png preview: a `#[test]` whose
// `Scene::build` registers the real spawner and whose `run` result is checked
// (LELEBEVY003 + LELEBEVY007 compliant).

#[derive(Component)]
pub struct Target;

pub struct Spawner;

pub fn spawn_target(spawner: &mut Spawner) {
    spawner.spawn((Target, Sprite));
}

#[cfg(test)]
mod tests {
    use lele_bevy_preview::run;
    use lele_bevy_preview::scene::{Kind, Scene, State};

    fn build(app: &mut App) {
        app.add_systems(Startup, super::spawn_target);
        app.add_systems(PostStartup, lele_bevy_preview::scene::bevy_systems::center_roots);
    }

    fn rest(_world: &mut World) {}

    #[test]
    fn target_ui_png_preview() {
        let scene = Scene {
            name: String::from("target"),
            kind: Kind::System,
            build,
            states: vec![State {
                label: String::from("rest"),
                apply: rest,
            }],
            timeline: None,
        };
        run(&scene, &Config::default(), "compliant_crate").expect("preview");
    }
}
