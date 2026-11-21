use derive_more::Deref;

use crate::pool;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deref)]
pub struct Sample(pub u32);

#[rustfmt::skip]
impl Sample {
    pub fn is_too_deep(self) -> bool { *self > pool::MAX_DEPTH }
}

#[cfg(test)]
mod tests {
    use super::Sample;

    #[test]
    fn test_usage() {
        assert!(!Sample(1).is_too_deep());
    }
}
