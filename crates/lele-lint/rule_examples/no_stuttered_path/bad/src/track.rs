use crate::meters;

pub struct Track {
    pub name: String,
    pub length: meters::Meters,
}

#[rustfmt::skip]
impl Track {
    pub fn laps_for(&self, distance: u32) -> u32 { distance / *self.length }
}

#[cfg(test)]
mod tests {
    use crate::meters;
    use crate::Track;

    #[test]
    fn test_usage() {
        let track = Track { name: "oval".to_string(), length: meters::Meters(400) };
        assert_eq!(track.laps_for(800), 2);
    }
}
