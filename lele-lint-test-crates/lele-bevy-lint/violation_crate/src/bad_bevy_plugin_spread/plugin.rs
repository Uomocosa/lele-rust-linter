// A Plugin whose build spawns UI lives one level below it (in `bevy_systems/`),
// so `dir_spawns_ui` is false; the system it registers still needs a scene
// preview (LELEBEVY006).

pub struct SpreadPlugin;

impl Plugin for SpreadPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, bad_bevy_plugin_spread::bevy_systems::spawn_hud);
    }
}
