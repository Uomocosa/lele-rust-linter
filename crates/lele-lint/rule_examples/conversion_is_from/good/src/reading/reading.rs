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
