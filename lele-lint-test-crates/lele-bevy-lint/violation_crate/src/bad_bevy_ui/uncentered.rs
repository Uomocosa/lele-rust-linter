// Previews a Component scene without centring it, so it renders in the top-left corner (LELEBEVY008).

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
    }

    fn rest(_world: &mut World) {}

    #[test]
    fn uncentered_ui_png_preview() {
        let scene = Scene {
            name: String::from("uncentered"),
            kind: Kind::Component,
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
