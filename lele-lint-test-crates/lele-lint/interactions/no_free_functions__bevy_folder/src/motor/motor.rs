use derive_more::Deref;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deref)]
pub struct Motor(pub u64);

#[rustfmt::skip]
impl Motor {
    pub fn speed(self) -> u64 { *self }
}

#[cfg(test)]
mod tests {
    use super::Motor;

    #[test]
    fn test_usage() {
        assert_eq!(Motor(5).speed(), 5);
    }
}
