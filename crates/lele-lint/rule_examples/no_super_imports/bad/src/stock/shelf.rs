use super::Item;

pub struct Shelf {
    pub label: String,
    pub items: Vec<Item>,
}

#[rustfmt::skip]
impl Shelf {
    pub fn total_price(&self) -> u32 { self.items.iter().map(|item| item.price).sum() }
}
