pub struct StarboardPlugin;

#[atomic_delegates]
impl StarboardPlugin {
    pub fn build(&self, app: &mut Vec<String>) {}
}

impl bevy::app::Plugin for StarboardPlugin {
    fn build(&self, app: &mut Vec<String>) {
        self.build(app)
    }
}

// no test_usage necessary
