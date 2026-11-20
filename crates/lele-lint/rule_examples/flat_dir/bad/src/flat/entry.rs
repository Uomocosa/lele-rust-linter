use derive_more::Deref;

#[derive(Deref)]
pub struct Entry(pub u64);

#[rustfmt::skip]
impl Entry {
    pub fn bump(&self) -> u64 { self.wrapping_add(1) }
}

#[cfg(test)]
mod tests {
    use super::Entry;

    #[test]
    fn test_usage() {
        assert_eq!(Entry(41).bump(), 42);
    }
}
