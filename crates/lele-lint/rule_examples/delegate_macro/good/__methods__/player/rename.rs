use crate::player;

pub fn rename(player: &mut player::Player, name: String) {
    let trimmed = name.trim().to_string();
    player.name = trimmed;
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
