use std::fmt;

pub struct Round {
    pub hits: u32,
    pub misses: u32,
}

impl Default for Round {
    fn default() -> Self {
        Self { hits: 0, misses: 0 }
    }
}

#[rustfmt::skip]
impl From<u32> for Round {
    fn from(hits: u32) -> Self { Self { hits, misses: 0 } }
}

#[rustfmt::skip]
impl fmt::Display for Round {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result { write!(formatter, "{}/{}", self.hits, self.misses) }
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
        assert_eq!(Round::from(1).score(), 10);
        assert_eq!(Round::default().to_string(), "0/0");
    }
}
