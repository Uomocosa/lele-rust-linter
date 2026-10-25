use derive_more::Deref;

#[derive(Deref)]
pub struct Session(pub u64);

#[rustfmt::skip]
impl Session {
    pub fn expiry(&self) -> u64 { std::time::SystemTime::now().elapsed().map_or(**self, |_| self.saturating_sub(1)) }
}
