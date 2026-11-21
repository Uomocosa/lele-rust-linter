use crate::limits;

pub struct Valve;

#[rustfmt::skip]
impl Valve {
    pub fn capped(flow: u64) -> u64 { flow.min(u64::from(limits::MAX_FLOW)) }
}

#[cfg(test)]
mod tests {
    use super::Valve;
    use crate::limits;

    #[test]
    fn test_usage() {
        assert_eq!(Valve::capped(99), u64::from(limits::MAX_FLOW));
    }
}
