use derive_more::Deref;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deref)]
pub struct Widget(pub u64);

#[rustfmt::skip]
impl Widget {
    pub fn id(self) -> u64 { *self }
}

#[cfg(test)]
mod tests {
    use super::Widget;

    #[test]
    fn test_usage() {
        assert_eq!(Widget(41).id(), 41);
    }
}
