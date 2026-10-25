pub struct Reading {
    pub sensor: String,
    pub celsius: i32,
}

impl Reading {
    pub fn at(sensor: &str, celsius: i32) -> Self {
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
        let reading = reading::Reading::at("kitchen", 21);
        assert_eq!(reading.celsius, 21);
    }
}
