pub struct Settings {
    pub host: String,
    pub port: u16,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 8080,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Settings;

    #[test]
    fn test_usage() {
        assert_eq!(Settings::default().port, 8080);
    }
}
