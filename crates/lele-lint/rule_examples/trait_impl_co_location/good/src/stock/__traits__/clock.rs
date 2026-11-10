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

    struct Demo {
        seconds: u64,
        ticks: u32,
    }

    impl Clock for Demo {
        fn now(&self) -> u64 {
            self.seconds + u64::from(self.ticks)
        }
    }

    #[test]
    fn test_usage() {
        let demo = Demo {
            seconds: 6,
            ticks: 1,
        };
        assert_eq!(demo.now(), 7);
    }
}
