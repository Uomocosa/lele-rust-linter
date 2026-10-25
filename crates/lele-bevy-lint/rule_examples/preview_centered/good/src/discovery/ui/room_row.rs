pub fn highlight(s: &mut S) {}

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
