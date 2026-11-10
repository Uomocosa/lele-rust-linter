use crate::stock;

impl stock::Clock for UnixTime {
    fn now(&self) -> u64 {
        self.seconds
    }
}

// no test_usage necessary
