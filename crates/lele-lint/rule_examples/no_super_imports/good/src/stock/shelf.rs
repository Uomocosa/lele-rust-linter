use crate::stock;

pub struct Shelf {
    pub label: String,
    pub items: Vec<stock::Item>,
}

#[rustfmt::skip]
impl Shelf {
    pub fn total_price(&self) -> u32 { self.items.iter().map(|item| item.price).sum() }
}

#[cfg(test)]
mod tests {
    use super::Shelf;
    use crate::stock::Item;

    #[test]
    fn test_usage() {
        let shelf = Shelf {
            label: "fruit".to_string(),
            items: vec![Item {
                name: "apple".to_string(),
                price: 3,
            }],
        };
        assert_eq!(shelf.total_price(), 3);
    }
}
