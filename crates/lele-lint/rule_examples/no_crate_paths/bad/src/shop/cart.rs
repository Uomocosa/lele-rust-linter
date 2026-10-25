pub struct Cart {
    pub owner: String,
    pub items: Vec<crate::stock::Item>,
}

#[rustfmt::skip]
impl Cart {
    pub fn total(&self) -> u32 { self.items.iter().map(|item| item.price).sum() }
}

#[cfg(test)]
mod tests {
    use crate::shop;
    use crate::stock;

    #[test]
    fn test_usage() {
        let cart = shop::Cart {
            owner: "ada".to_string(),
            items: vec![stock::Item {
                name: "apple".to_string(),
                price: 3,
            }],
        };
        assert_eq!(cart.total(), 3);
    }
}
