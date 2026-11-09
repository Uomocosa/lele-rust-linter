use derive_more::Deref;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deref)]
pub struct Duration(pub u64);

impl Duration {
    pub fn from_secs(secs: u64) -> Self {
        Self(secs * 1000)
    }

    pub fn from_millis(millis: u64) -> Self {
        Self(millis)
    }
}

#[cfg(test)]
mod tests {
    use crate::duration;

    #[test]
    fn test_usage() {
        let duration = duration::Duration::from_secs(1);
        assert_eq!(*duration, 1000);
    }
}
