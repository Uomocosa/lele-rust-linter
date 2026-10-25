use derive_more::Deref;

#[derive(Deref)]
pub struct Session(pub u64);

#[rustfmt::skip]
impl Session {
    pub fn expiry(&self) -> u64 { self.saturating_sub(1) }
}

#[cfg(test)]
mod tests {
    use super::Session;

    #[test]
    fn test_usage() {
        assert_eq!(Session(3).expiry(), 2);
    }
}
