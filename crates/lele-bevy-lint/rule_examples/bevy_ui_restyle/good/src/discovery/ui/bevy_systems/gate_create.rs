pub fn build(app: &mut App) {
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
        app.add_systems(PostStartup, lele_bevy_preview::scene::bevy_systems::center_roots);
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
