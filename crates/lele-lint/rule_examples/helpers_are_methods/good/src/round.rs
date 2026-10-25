pub struct Round {
    pub hits: u32,
    pub misses: u32,
}

#[rustfmt::skip]
impl Round {
    pub fn score(&self) -> u32 { Self::base(self.hits).saturating_sub(self.misses) }
    pub(crate) fn base(hits: u32) -> u32 { hits.saturating_mul(10) }
}

#[cfg(test)]
mod tests {
    use super::Round;

    #[test]
    fn test_usage() {
        assert_eq!(Round { hits: 1, misses: 0 }.score(), 10);
    }
}
