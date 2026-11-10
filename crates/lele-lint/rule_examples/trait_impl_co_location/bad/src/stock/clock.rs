use crate::stock;

impl stock::Clock for UnixTime {
    fn now(&self) -> u64 {
        self.seconds
    }
}

impl stock::Clock for Dial {
    fn now(&self) -> u64 {
        u64::from(self.turns)
    }
}

// no test_usage necessary
