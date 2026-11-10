use crate::stock;
use std::fmt;

pub struct UnixTime {
    pub seconds: u64,
    pub nanos: u32,
}

#[rustfmt::skip]
impl stock::Clock for UnixTime {
    fn now(&self) -> u64 { self.seconds }
}

#[rustfmt::skip]
impl fmt::Display for UnixTime {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result { write!(formatter, "{}s", self.seconds) }
}
