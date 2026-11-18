pub struct Query;

pub fn tick(_q: Query) {}

pub fn build(app: &mut App) {
    app.add_systems(Update, tick);
}

#[cfg(test)]
mod tests {
    use super::Query;
    use super::tick;
    use lele_bevy_preview::run;
    use lele_bevy_preview::scene::{Kind, Scene, State};

    #[test]
    fn tick_ui_png_preview() {
        let scene = Scene {
            name: String::from("tick"),
            kind: Kind::System,
            build: |app: &mut App| {
                app.add_systems(Update, tick);
                app.add_systems(
                    PostStartup,
                    lele_bevy_preview::scene::bevy_systems::center_roots,
                );
            },
            states: vec![
                State {
                    label: String::from("blank"),
                    apply: |_world: &mut World| {},
                },
                State {
                    label: String::from("typed"),
                    apply: |_world: &mut World| {},
                },
            ],
            timeline: None,
        };
        run(&scene, &Config::default()).expect("preview");
    }
}
