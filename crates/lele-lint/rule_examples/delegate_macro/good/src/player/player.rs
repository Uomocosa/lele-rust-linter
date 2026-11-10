use std::fmt;

use atomic_delegate_macros::atomic_delegates;

pub struct Player {
    pub name: String,
    pub health: u32,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            name: String::new(),
            health: 100,
        }
    }
}

impl Player {
    pub fn new(name: String, health: u32) -> Self {
        Self { name, health }
    }
}

#[atomic_delegates]
impl Player {
    pub fn from_save(name: String, health: u32) -> Self {}
}

#[rustfmt::skip]
impl fmt::Display for Player {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result { write!(formatter, "{} ({} hp)", self.name, self.health) }
}

#[atomic_delegates]
impl Player {
    pub fn heal(&mut self, amount: u32) {}
    pub fn rename(&mut self, name: String) {}
}

#[rustfmt::skip]
impl Player {
    pub fn banner(&self) -> String { format!("{} ({} hp)", self.name, self.health) }
}

#[cfg(test)]
mod tests {
    use crate::player;

    #[test]
    fn test_usage() {
        let mut player = player::Player::new("ada".to_string(), 50);
        player.heal(5);
        assert_eq!(player.health, 55);
        player.rename("  bob  ".to_string());
        assert_eq!(player.name, "bob");
        let restored = player::Player::from_save("eve".to_string(), 250);
        assert_eq!(restored.health, 100);
        assert_eq!(restored.banner(), "eve (100 hp)");
    }
}
