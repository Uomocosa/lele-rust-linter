use crate::player;

pub fn heal(player: &mut player::Player, amount: u32) {
    let healed = player.health.saturating_add(amount);
    player.health = capped(healed);
}

fn capped(health: u32) -> u32 {
    health.min(100)
}

#[cfg(test)]
mod tests {
    use super::heal;
    use crate::player::Player;

    #[test]
    fn test_usage() {
        let mut player = Player {
            name: String::new(),
            health: 10,
        };
        heal(&mut player, 5);
        assert_eq!(player.health, 15);
    }
}
