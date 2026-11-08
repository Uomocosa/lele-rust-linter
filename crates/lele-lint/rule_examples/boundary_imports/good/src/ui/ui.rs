use derive_more::Deref;

#[derive(Deref)]
pub struct Ui(pub u64);

#[rustfmt::skip]
impl Ui {
    pub fn bump(&self) -> u64 { self.wrapping_add(1) }
}

#[cfg(test)]
mod tests {
    use super::Ui;

    #[test]
    fn test_usage() {
        assert_eq!(Ui(41).bump(), 42);
    }
}
