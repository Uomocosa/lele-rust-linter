pub struct LobbyRow {
    pub name: String,
    pub occupants: usize,
}

#[rustfmt::skip]
impl LobbyRow {
    pub fn is_full(&self) -> bool { self.occupants >= 2 }
}

#[cfg(test)]
mod tests {
    use super::LobbyRow;

    #[test]
    fn test_usage() {
        let row = LobbyRow {
            name: "alpha".to_string(),
            occupants: 2,
        };
        assert!(row.is_full());
    }
}
