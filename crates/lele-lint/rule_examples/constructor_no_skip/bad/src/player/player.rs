pub struct Player {
    pub name: String,
    pub health: u32,
}

#[rustfmt::skip]
impl Default for Player {
    fn default() -> Self {
        Self {
            name: String::new(),
            health: 100,
        }
    }
}
