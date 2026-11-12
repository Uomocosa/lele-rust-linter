pub struct Reading {
    pub sensor: String,
    pub celsius: i32,
}

impl From<reading::SensorName> for Reading {
    fn from(sensor: reading::SensorName) -> Self {
        Self {
            sensor: (*sensor).clone(),
            celsius: 21,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::reading;

    #[test]
    fn test_usage() {
        let reading = reading::Reading::from(reading::SensorName("kitchen".to_string()));
        assert_eq!(reading.celsius, 21);
    }
}
