// Registers its spawner through a chained `add_systems` call: the inner call's
// systems are production roots too, so the spawn needs a png preview (LELEBEVY003).

pub struct Spawner;
pub struct App;

pub fn wire(app: &mut App) {
    app.add_systems(Startup, spawn_hud).add_systems(Update, tick_hud);
}

fn spawn_hud(spawner: &mut Spawner) {
    spawner.spawn((Text,));
}

fn tick_hud() {}
