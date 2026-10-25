pub struct FreenetClient {
    pub host: String,
    pub port: u16,
}

#[rustfmt::skip]
impl FreenetClient {
    pub fn url(&self) -> String { format!("ws://{}:{}", self.host, self.port) }
}

#[cfg(test)]
mod tests {
    use crate::freenet;

    #[test]
    fn test_usage() {
        let client = freenet::FreenetClient {
            host: "localhost".to_string(),
            port: 7509,
        };
        assert_eq!(client.url(), "ws://localhost:7509");
    }
}
