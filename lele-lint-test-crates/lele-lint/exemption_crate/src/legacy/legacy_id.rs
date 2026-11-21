use derive_more::Deref;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deref)]
pub struct LegacyId(pub u64);

// generated mirror of the rule book: kept verbatim.

#[rustfmt::skip]
impl LegacyId {
    pub fn as_u64(self) -> u64 { *self }
}

#[cfg(test)]
mod tests {
    use crate::legacy::LegacyId;

    #[test]
    fn test_usage() {
        let id = LegacyId(7);
        assert_eq!(*id, 7);
        assert_eq!(id.as_u64(), 7);
    }
}
