pub fn setup(s: &mut S) { s.spawn((Node,)); }

pub fn tick(time: Res<Time>) { let _ = time.delta_secs(); }

#[cfg(test)]
mod tests {
    use lele_bevy_preview::run;
    use lele_bevy_preview::scene::{Kind, Scene, State, Timeline};

    fn build(app: &mut App) {
        app.add_systems(Startup, super::setup).add_systems(Update, super::tick);
        app.add_systems(PostStartup, lele_bevy_preview::scene::bevy_systems::center_roots);
    }

    fn rest(_world: &mut World) {}

    fn scene(timeline: Option<Timeline>) -> Scene {
        Scene {
            name: String::from("sync_room_list"),
            kind: Kind::System,
            build,
            states: vec![State { label: String::from("rest"), apply: rest }],
            timeline,
        }
    }

    #[test]
    fn sync_room_list_ui_png_preview() {
        run(&scene(None), &Config::default(), "x").expect("preview");
    }

    #[test]
    fn sync_room_list_ui_mp4_preview() {
        let clip = Timeline { label: String::from("tick"), frames: 30, fps: 30, apply: rest };
        run(&scene(Some(clip)), &Config::default(), "x").expect("preview");
    }
}
