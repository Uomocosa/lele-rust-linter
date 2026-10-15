// A registered system that restyles UI (`&mut BackgroundColor`) but ships no
// preview that renders it (LELEBEVY042).

pub struct Spawner;

pub fn setup(button: Query<&mut BackgroundColor>) {
    let _ = button;
}
