use derive_more::Deref;

use crate::pool;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deref)]
pub struct Diver(pub u32);

#[rustfmt::skip]
impl Diver {
    pub fn is_too_deep(self) -> bool { *self > pool::MAX_DEPTH }
}

#[cfg(test)]
mod tests {
    use super::Diver;

    #[test]
    fn test_usage() {
        assert!(Diver(9).is_too_deep());
    }
}
