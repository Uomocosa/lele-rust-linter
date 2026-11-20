use derive_more::Deref;

#[derive(Deref)]
pub struct Nested(pub u64);

#[rustfmt::skip]
impl Nested {
    pub fn bump(&self) -> u64 { self.wrapping_add(1) }
}

#[cfg(test)]
mod tests {
    use super::Nested;

    #[test]
    fn test_usage() {
        assert_eq!(Nested(41).bump(), 42);
    }
}
