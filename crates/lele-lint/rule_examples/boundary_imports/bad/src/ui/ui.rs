use derive_more::Deref;

#[derive(Deref)]
pub struct Ui(pub u64);

#[rustfmt::skip]
impl Ui {
    pub fn bump(&self) -> u64 { self.wrapping_add(1) }
}
