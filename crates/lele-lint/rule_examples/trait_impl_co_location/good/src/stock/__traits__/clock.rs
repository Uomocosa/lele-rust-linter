pub trait Clock {
    fn now(&self) -> u64;
}

#[rustfmt::skip]
impl<T> Clock for Vec<T> {
    fn now(&self) -> u64 { u64::try_from(self.len()).unwrap_or(0) }
}

#[cfg(test)]
mod tests {
    use super::Clock;

    #[test]
    fn test_usage() {
        assert_eq!(vec![1_u8, 2_u8, 3_u8].now(), 3);
    }
}
