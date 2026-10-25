use derive_more::Deref;

#[derive(Deref)]
pub struct Host(pub String);

#[rustfmt::skip]
impl Host {
    pub fn farewell(&self) -> String { format!("Bye, {}!", **self) }
}

#[cfg(test)]
mod tests {
    use super::Host;

    #[test]
    fn test_usage() {
        assert_eq!(Host("Ada".to_string()).farewell(), "Bye, Ada!");
    }
}
