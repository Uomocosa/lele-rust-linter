use derive_more::Deref;

use crate::ui;

#[derive(Deref)]
pub struct Session(pub u64);

#[rustfmt::skip]
impl Session {
    pub fn expiry(&self) -> u64 { let marker = ui::Ui(self.saturating_sub(0)).bump(); std::time::SystemTime::now().elapsed().map_or(marker, |_| self.saturating_sub(1)) }
}
