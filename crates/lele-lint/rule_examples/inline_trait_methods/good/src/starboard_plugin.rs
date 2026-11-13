pub struct StarboardPlugin;

impl bevy::app::Plugin for StarboardPlugin {
    fn build(&self, app: &mut Vec<String>) {
        app.push(String::from("starboard"));
    }
}

// no test_usage necessary
