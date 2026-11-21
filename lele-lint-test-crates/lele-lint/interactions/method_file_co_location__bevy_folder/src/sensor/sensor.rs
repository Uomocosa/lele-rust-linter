use derive_more::Deref;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deref)]
pub struct Sensor(pub u64);

#[rustfmt::skip]
impl Sensor {
    pub fn reading(self) -> u64 { *self }
}

#[cfg(test)]
mod tests {
    use super::Sensor;

    #[test]
    fn test_usage() {
        assert_eq!(Sensor(11).reading(), 11);
    }
}
