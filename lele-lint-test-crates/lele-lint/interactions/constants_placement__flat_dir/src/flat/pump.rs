use crate::limits;

pub struct Pump;

#[rustfmt::skip]
impl Pump {
    pub fn capped(flow: u64) -> u64 { flow.min(u64::from(limits::MAX_FLOW)) }
}

#[cfg(test)]
mod tests {
    use super::Pump;
    use crate::limits;

    #[test]
    fn test_usage() {
        assert_eq!(Pump::capped(3), 3);
    }
}
