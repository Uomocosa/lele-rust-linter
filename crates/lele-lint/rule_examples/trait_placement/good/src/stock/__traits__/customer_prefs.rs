pub trait CustomerPrefs {
    fn prefs_path(&self) -> String;
}

#[rustfmt::skip]
impl CustomerPrefs for String {
    fn prefs_path(&self) -> String { format!("prefs/{self}") }
}

#[cfg(test)]
mod tests {
    use super::CustomerPrefs;

    #[test]
    fn test_usage() {
        assert_eq!("house".to_string().prefs_path(), "prefs/house");
    }
}
