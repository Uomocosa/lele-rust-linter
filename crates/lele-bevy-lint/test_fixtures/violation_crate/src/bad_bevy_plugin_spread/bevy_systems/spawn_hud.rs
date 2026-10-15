pub struct Node;

pub fn spawn_hud(spawner: &mut Spawner) {
    spawner.spawn((Node,));
}
