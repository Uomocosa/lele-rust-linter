use crate::player;

pub fn rename(player: &mut player::Player, name: String) {
    player.name = name;
}
