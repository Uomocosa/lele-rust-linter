// Test-only file mounted via `#[cfg(test)] #[path]` in lib.rs: its UI spawns are fixtures, not production.

fn spawn_fixture(mut commands: Commands) {
    commands.spawn((Node,));
}

#[test]
fn fixture_registers_a_system() {
    let mut app = App::new();
    app.add_systems(Startup, spawn_fixture);
}
