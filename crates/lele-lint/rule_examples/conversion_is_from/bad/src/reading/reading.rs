pub struct Reading {
    pub sensor: String,
    pub celsius: i32,
}

impl Reading {
    pub fn from_sensor(sensor: &str) -> Self {
        Self {
            sensor: sensor.to_string(),
            celsius: 21,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::reading;

    #[test]
    fn test_usage() {
        let reading = reading::Reading::from_sensor("kitchen");
        assert_eq!(reading.celsius, 21);
    }
}
