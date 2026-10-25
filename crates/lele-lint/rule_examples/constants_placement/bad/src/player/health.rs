use derive_more::Deref;

use crate::constants;

#[derive(Deref)]
pub struct Health(pub u32);

#[rustfmt::skip]
impl Health {
    pub fn clamped(&self) -> u32 { (**self).min(constants::MAX_HEALTH) }
}

#[cfg(test)]
mod tests {
    use super::Health;

    #[test]
    fn test_usage() {
        assert_eq!(Health(150).clamped(), 100);
    }
}
