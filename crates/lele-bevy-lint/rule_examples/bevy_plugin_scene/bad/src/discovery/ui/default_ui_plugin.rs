pub fn spawn_ui(s: &mut S) { s.spawn((Node,)); }

pub struct DefaultUiPlugin;

impl Plugin for DefaultUiPlugin {
    fn build(&self, app: &mut A) { spawn_ui(app); }
}
