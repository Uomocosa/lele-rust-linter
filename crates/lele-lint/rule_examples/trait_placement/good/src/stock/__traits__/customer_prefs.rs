pub trait CustomerPrefs {
    fn prefs_path(&self) -> String;
}

#[cfg(test)]
mod tests {
    use super::CustomerPrefs;

    struct Demo {
        suffix: String,
        created_on: u64,
    }

    impl CustomerPrefs for Demo {
        fn prefs_path(&self) -> String {
            format!("prefs/{}-{}", self.suffix, self.created_on)
        }
    }

    #[test]
    fn test_usage() {
        let demo = Demo {
            suffix: "house".to_string(),
            created_on: 4,
        };
        assert_eq!(demo.prefs_path(), "prefs/house-4");
    }
}
