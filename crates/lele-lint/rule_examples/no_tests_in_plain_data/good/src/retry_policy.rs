pub struct RetryPolicy {
    pub attempts: u32,
    pub delay_ms: u64,
}

impl RetryPolicy {
    pub fn new(attempts: u32) -> Self {
        let clamped = match attempts {
            0 => 1,
            n => n,
        };
        Self {
            attempts: clamped,
            delay_ms: 100,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::RetryPolicy;

    #[test]
    fn test_usage() {
        assert_eq!(RetryPolicy::new(0).attempts, 1);
    }
}
