use crate::player;

pub fn heal(player: &mut player::Player, amount: u32) {
    let healed = player.health.saturating_add(amount);
    player.health = healed.min(100);
}

#[cfg(test)]
mod tests {
    use crate::player;

    #[test]
    fn test_usage() {
        let mut player = player::Player { name: String::new(), health: 10 };
        player::heal(&mut player, 5);
        assert_eq!(player.health, 15);
    }
}
