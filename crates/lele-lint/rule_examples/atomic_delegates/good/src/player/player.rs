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

#[rustfmt::skip]
impl Player {
    pub fn is_alive(&self) -> bool { self.health > 0 }
}

impl Player {
    pub fn status(&self) -> &'static str {
        match self.health {
            0 => "down",
            _ => "up",
        }
    }
    pub fn label(&self) -> String {
        let prefix = "player";
        format!("{prefix}: {}", self.name)
    }
}

#[cfg(test)]
mod tests {
    use crate::player;

    #[test]
    fn test_usage() {
        assert!(player::Player::default().is_alive());
    }
}
