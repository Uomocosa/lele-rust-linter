pub struct Reading {
    pub sensor: String,
    pub celsius: i32,
}

impl Reading {
    pub fn from_sensor_and_celsius(sensor: &str, celsius: i32) -> Self {
        Self {
            sensor: sensor.to_string(),
            celsius,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::reading;

    #[test]
    fn test_usage() {
        let reading = reading::Reading::from_sensor_and_celsius("kitchen", 21);
        assert_eq!(reading.celsius, 21);
    }
}
