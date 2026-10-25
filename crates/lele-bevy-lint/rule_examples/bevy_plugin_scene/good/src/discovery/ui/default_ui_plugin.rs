pub fn spawn_ui(s: &mut S) { s.spawn((Node,)); }

pub struct DefaultUiPlugin;

impl Plugin for DefaultUiPlugin {
    fn build(&self, app: &mut A) { spawn_ui(app); }
}

#[cfg(test)]
mod tests {
    use lele_bevy_preview::run;
    use lele_bevy_preview::scene::{Kind, Scene, State};

    fn build(app: &mut App) {
        app.add_plugins(super::DefaultUiPlugin);
    }

    fn rest(_world: &mut World) {}

    fn scene() -> Scene {
        Scene {
            name: String::from("default_ui_plugin"),
            kind: Kind::Plugin,
            build,
            states: vec![State { label: String::from("rest"), apply: rest }],
            timeline: None,
        }
    }

    #[test]
    fn default_ui_plugin_ui_png_preview() {
        run(&scene(), &Config::default(), "x").expect("preview");
    }

    #[test]
    fn default_ui_plugin_ui_scene_preview() {
        run(&scene(), &Config::default(), "x").expect("preview");
    }
}
