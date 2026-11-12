use crate::player;

pub fn from_save(name: String, health: u32) -> player::Player {
    player::Player {
        name,
        health: health.min(100),
    }
}

#[cfg(test)]
mod tests {
    use super::from_save;

    #[test]
    fn test_usage() {
        let player = from_save("ada".to_string(), 250);
        assert_eq!(player.name, "ada");
        assert_eq!(player.health, 100);
    }
}
