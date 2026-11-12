use crate::player;

pub fn rename(player: &mut player::Player, name: String) {
    if name.trim().is_empty() {
        return;
    }
    player.name = name.trim().to_string();
}

#[cfg(test)]
mod tests {
    use crate::player;

    #[test]
    fn test_usage() {
        let mut player = player::Player {
            name: String::new(),
            health: 10,
        };
        rename(&mut player, "  ada  ".to_string());
        assert_eq!(player.name, "ada");
    }
}
